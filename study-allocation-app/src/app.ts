import { allocate, getConfig, mockVolunteers, setConfig } from "./api";
import type {
  AllocationOutput,
  AppConfig,
  PreferenceMode,
  VolunteerInput,
} from "./types";
import {
  emptyVolunteers,
  MAX_VOLUNTEERS,
  totalCapacity,
} from "./types";

type TabId = "input" | "result" | "settings";

interface AppState {
  config: AppConfig;
  volunteers: VolunteerInput[];
  result: AllocationOutput | null;
  activeTab: TabId;
  errors: string[];
}

const state: AppState = {
  config: {
    subjects: [
      { name: "국어", capacity: 4 },
      { name: "수학", capacity: 4 },
      { name: "영어", capacity: 4 },
      { name: "사탐", capacity: 4 },
      { name: "과탐", capacity: 4 },
    ],
    preference_mode: "two",
  },
  volunteers: emptyVolunteers(20),
  result: null,
  activeTab: "input",
  errors: [],
};

const root = document.getElementById("app")!;

function subjectOptions(config: AppConfig): string[] {
  return config.subjects.map((s) => s.name.trim()).filter(Boolean);
}

function showErrors(messages: string[]) {
  state.errors = messages;
  render();
}

function clearErrors() {
  state.errors = [];
}

function resizeVolunteers() {
  const count = totalCapacity(state.config);
  const next = emptyVolunteers(count);
  for (let i = 0; i < Math.min(count, state.volunteers.length); i += 1) {
    next[i] = { ...state.volunteers[i] };
  }
  state.volunteers = next;
}

function setTab(tab: TabId) {
  state.activeTab = tab;
  render();
}

async function loadInitialConfig() {
  try {
    state.config = await getConfig();
    resizeVolunteers();
  } catch (error) {
    showErrors([String(error)]);
  }
}

async function onSaveSettings() {
  try {
    clearErrors();
    state.config = await setConfig(state.config);
    resizeVolunteers();
    state.result = null;
    setTab("input");
  } catch (error) {
    showErrors(String(error).split("\n"));
  }
}

async function onRunAllocation() {
  try {
    clearErrors();
    state.result = await allocate(state.config, state.volunteers);
    setTab("result");
  } catch (error) {
    showErrors(String(error).split("\n"));
    setTab("input");
  }
}

async function onMockData() {
  try {
    clearErrors();
    state.volunteers = await mockVolunteers(state.config, 42);
    state.result = null;
    render();
  } catch (error) {
    showErrors(String(error).split("\n"));
  }
}

function onClearData() {
  clearErrors();
  state.volunteers = emptyVolunteers(totalCapacity(state.config));
  state.result = null;
  setTab("input");
}

function renderErrors(): string {
  if (state.errors.length === 0) {
    return "";
  }
  return `
    <section class="errors">
      <strong>입력 오류</strong>
      <ul>${state.errors.map((e) => `<li>${escapeHtml(e)}</li>`).join("")}</ul>
    </section>
  `;
}

function renderTabs(): string {
  const tabs: { id: TabId; label: string }[] = [
    { id: "input", label: "나누미 입력" },
    { id: "result", label: "배정 결과" },
    { id: "settings", label: "설정" },
  ];
  return `
    <nav class="tabs">
      ${tabs
        .map(
          (tab) => `
        <button
          type="button"
          class="tab ${state.activeTab === tab.id ? "active" : ""}"
          data-tab="${tab.id}"
        >
          ${tab.label}
        </button>`,
        )
        .join("")}
    </nav>
  `;
}

function renderInputTab(): string {
  const options = subjectOptions(state.config);
  const showThird = state.config.preference_mode === "three";
  const prefHeaders = showThird
    ? "<th>1지망</th><th>2지망</th><th>3지망</th>"
    : "<th>1지망</th><th>2지망</th>";

  const rows = state.volunteers
    .map(
      (vol, index) => `
      <tr>
        <td>${index + 1}</td>
        <td>
          <input type="text" data-field="name" data-index="${index}" value="${escapeAttr(vol.name)}" />
        </td>
        ${renderPrefCell(index, "pref1", vol.pref1, options, true)}
        ${renderPrefCell(index, "pref2", vol.pref2, options, false)}
        ${showThird ? renderPrefCell(index, "pref3", vol.pref3 ?? "", options, false) : ""}
      </tr>`,
    )
    .join("");

  return `
    <section class="panel">
      <p class="hint">
        총 ${totalCapacity(state.config)}명 · 지망 ${state.config.preference_mode === "three" ? "1~3" : "1~2"} · 최대 ${MAX_VOLUNTEERS}명
      </p>
      <div class="table-wrap">
        <table class="data-table">
          <thead>
            <tr>
              <th>#</th>
              <th>이름</th>
              ${prefHeaders}
            </tr>
          </thead>
          <tbody>${rows}</tbody>
        </table>
      </div>
      <div class="actions">
        <button type="button" class="btn secondary" id="btn-mock">샘플 데이터</button>
        <button type="button" class="btn primary" id="btn-run">배정 실행</button>
        <button type="button" class="btn ghost" id="btn-clear">초기화</button>
      </div>
    </section>
  `;
}

function renderPrefCell(
  index: number,
  field: keyof VolunteerInput,
  value: string,
  options: string[],
  required: boolean,
): string {
  const items = options
    .map(
      (name) =>
        `<option value="${escapeAttr(name)}" ${value === name ? "selected" : ""}>${escapeHtml(name)}</option>`,
    )
    .join("");
  return `
    <td>
      <select data-field="${field}" data-index="${index}" ${required ? "required" : ""}>
        <option value="" ${value === "" ? "selected" : ""}>—</option>
        ${items}
      </select>
    </td>
  `;
}

function renderResultTab(): string {
  if (!state.result) {
    return `
      <section class="panel empty-state">
        <p>배정 결과가 없습니다. 먼저 나누미 입력 후 배정을 실행하세요.</p>
      </section>
    `;
  }

  const showThird = state.config.preference_mode === "three";
  const resultRows = state.result.rows
    .map(
      (row) => `
      <tr>
        <td>${row.id}</td>
        <td>${escapeHtml(row.name)}</td>
        <td>${escapeHtml(row.pref1 || "—")}</td>
        <td>${escapeHtml(row.pref2 || "—")}</td>
        ${showThird ? `<td>${escapeHtml(row.pref3 || "—")}</td>` : ""}
        <td>${escapeHtml(row.assigned)}</td>
        <td>${escapeHtml(row.match_label)}</td>
      </tr>`,
    )
    .join("");

  const summaryRows = state.result.summary
    .map(
      (row) => `
      <tr>
        <td>${escapeHtml(row.subject)}</td>
        <td>${row.capacity}</td>
        <td>${row.count}</td>
        <td>${escapeHtml(row.names)}</td>
      </tr>`,
    )
    .join("");

  return `
    <section class="panel">
      <h2>배정 결과</h2>
      <div class="table-wrap short">
        <table class="data-table">
          <thead>
            <tr>
              <th>#</th>
              <th>이름</th>
              <th>1지망</th>
              <th>2지망</th>
              ${showThird ? "<th>3지망</th>" : ""}
              <th>배정</th>
              <th>일치</th>
            </tr>
          </thead>
          <tbody>${resultRows}</tbody>
        </table>
      </div>

      <h2>프로그램별 요약</h2>
      <div class="table-wrap short">
        <table class="data-table">
          <thead>
            <tr>
              <th>프로그램</th>
              <th>정원</th>
              <th>배정</th>
              <th>멤버</th>
            </tr>
          </thead>
          <tbody>${summaryRows}</tbody>
        </table>
      </div>
    </section>
  `;
}

function renderSettingsTab(): string {
  const modeTwoChecked = state.config.preference_mode === "two" ? "checked" : "";
  const modeThreeChecked = state.config.preference_mode === "three" ? "checked" : "";

  const subjectRows = state.config.subjects
    .map(
      (subject, index) => `
      <tr>
        <td>
          <input
            type="text"
            data-subject-name="${index}"
            value="${escapeAttr(subject.name)}"
            placeholder="프로그램명"
          />
        </td>
        <td>
          <input
            type="number"
            min="1"
            max="${MAX_VOLUNTEERS}"
            data-subject-capacity="${index}"
            value="${subject.capacity}"
          />
        </td>
        <td>
          <button type="button" class="btn ghost small" data-remove-subject="${index}">삭제</button>
        </td>
      </tr>`,
    )
    .join("");

  return `
    <section class="panel">
      <h2>지망 설정</h2>
      <div class="radio-row">
        <label>
          <input type="radio" name="pref-mode" value="two" ${modeTwoChecked} />
          1~2지망
        </label>
        <label>
          <input type="radio" name="pref-mode" value="three" ${modeThreeChecked} />
          1~3지망
        </label>
      </div>

      <h2>프로그램 · 정원</h2>
      <p class="hint">총 인원 ${totalCapacity(state.config)}명 / 최대 ${MAX_VOLUNTEERS}명</p>
      <div class="table-wrap short">
        <table class="data-table">
          <thead>
            <tr>
              <th>프로그램</th>
              <th>정원</th>
              <th></th>
            </tr>
          </thead>
          <tbody>${subjectRows}</tbody>
        </table>
      </div>
      <div class="actions">
        <button type="button" class="btn secondary" id="btn-add-subject">프로그램 추가</button>
        <button type="button" class="btn primary" id="btn-save-settings">설정 저장</button>
      </div>
    </section>
  `;
}

function renderPanel(): string {
  switch (state.activeTab) {
    case "input":
      return renderInputTab();
    case "result":
      return renderResultTab();
    case "settings":
      return renderSettingsTab();
    default:
      return "";
  }
}

function render() {
  root.innerHTML = `
    <header class="header">
      <h1>나눔교실 프로그램팀 배정</h1>
      <p class="subtitle">선호도 기반 최적 배정 (헝가리안 알고리즘)</p>
    </header>
    ${renderTabs()}
    ${renderErrors()}
    ${renderPanel()}
    <footer class="footer">
      <p>문의: 서울대학교 프로네시스 나눔실천단 38기 재무부장 정근규</p>
    </footer>
  `;
  bindEvents();
}

function bindEvents() {
  root.querySelectorAll("[data-tab]").forEach((el) => {
    el.addEventListener("click", () => {
      setTab(el.getAttribute("data-tab") as TabId);
    });
  });

  document.getElementById("btn-run")?.addEventListener("click", () => {
    void onRunAllocation();
  });
  document.getElementById("btn-mock")?.addEventListener("click", () => {
    void onMockData();
  });
  document.getElementById("btn-clear")?.addEventListener("click", onClearData);
  document.getElementById("btn-save-settings")?.addEventListener("click", () => {
    void onSaveSettings();
  });
  document.getElementById("btn-add-subject")?.addEventListener("click", () => {
    state.config.subjects.push({ name: "", capacity: 1 });
    render();
  });

  root.querySelectorAll("[data-remove-subject]").forEach((el) => {
    el.addEventListener("click", () => {
      const index = Number(el.getAttribute("data-remove-subject"));
      if (state.config.subjects.length <= 1) {
        showErrors(["프로그램은 1개 이상 필요합니다."]);
        return;
      }
      state.config.subjects.splice(index, 1);
      render();
    });
  });

  root.querySelectorAll("[data-subject-name]").forEach((el) => {
    el.addEventListener("input", () => {
      const index = Number(el.getAttribute("data-subject-name"));
      state.config.subjects[index].name = (el as HTMLInputElement).value;
    });
  });

  root.querySelectorAll("[data-subject-capacity]").forEach((el) => {
    el.addEventListener("input", () => {
      const index = Number(el.getAttribute("data-subject-capacity"));
      state.config.subjects[index].capacity = Number(
        (el as HTMLInputElement).value,
      );
      const hint = root.querySelector(".hint");
      if (hint && state.activeTab === "settings") {
        hint.textContent = `총 인원 ${totalCapacity(state.config)}명 / 최대 ${MAX_VOLUNTEERS}명`;
      }
    });
  });

  root.querySelectorAll('input[name="pref-mode"]').forEach((el) => {
    el.addEventListener("change", () => {
      state.config.preference_mode = (el as HTMLInputElement)
        .value as PreferenceMode;
      render();
    });
  });

  root.querySelectorAll("[data-field]").forEach((el) => {
    const handler = () => {
      const index = Number(el.getAttribute("data-index"));
      const field = el.getAttribute("data-field") as keyof VolunteerInput;
      state.volunteers[index][field] = (el as HTMLInputElement).value;
    };
    el.addEventListener("input", handler);
    el.addEventListener("change", handler);
  });
}

function escapeHtml(value: string): string {
  return value
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;");
}

function escapeAttr(value: string): string {
  return escapeHtml(value);
}

void loadInitialConfig().then(render);
