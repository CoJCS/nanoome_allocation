# 나눔교실 프로그램팀 배정 (Tauri)

macOS / Windows용 경량 데스크톱 앱입니다. 헝가리안 알고리즘(LSAP)으로
봉사자–프로그램 배정을 수행합니다.

## 요구 사항

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://www.rust-lang.org/tools/install)

## 개발

```bash
cd study-allocation-app
npm install
npm run tauri dev
```

## 빌드

```bash
npm run tauri build
```

빌드 산출물:

- macOS: `src-tauri/target/release/bundle/macos/나눔교실 프로그램팀 배정.app`
  - **더블클릭으로 바로 실행** (DMG 설치 불필요)
  - Applications 폴더로 복사해 두면 Launchpad에서도 실행 가능
- Windows: `src-tauri/target/release/bundle/nsis/` 또는 `msi/`

> Cursor 등 일부 환경에서는 `target`이 프로젝트 밖 임시 폴더에 생길 수 있습니다.
> 로컬 터미널에서 `npm run tauri build`를 실행하면 위 경로에 생성됩니다.

## 기능

- **나누미 입력**: 화면에서 이름·지망 입력 (최대 30명)
- **설정**: 프로그램·정원·1~2/1~3지망 (앱 내 설정 화면, JSON 저장)
- **배정 실행**: Rust 헝가리안 알고리즘
- **결과**: 배정표 + 프로그램별 요약 (화면 표시만)

## 기본 설정

- 프로그램: 국어·수학·영어·사탐·과탐 (각 4명, 총 20명)
- 지망: 1~2지망

설정은 OS 앱 설정 폴더의 `config.json`에 저장됩니다.

## GitHub Actions (Mac / Windows 동시 빌드)

저장소 루트의 [`.github/workflows/tauri-build.yml`](../.github/workflows/tauri-build.yml)가
matrix로 macOS(Apple Silicon·Intel)와 Windows를 각각 빌드합니다.

### 사전 설정 (GitHub)

1. 이 프로젝트를 GitHub 저장소에 push
2. **Settings → Actions → General → Workflow permissions**
   - **Read and write permissions** 선택 (태그 Release 업로드용)

### 실행 방법

| 방법 | 결과 |
|------|------|
| **Actions** → `Tauri build` → **Run workflow** | 3개 Artifacts (수동 배포용) |
| `git tag v0.1.0 && git push origin v0.1.0` | Artifacts + **Draft Release** |

### Artifacts 받기

워크플로 완료 후 실행(run) 페이지 하단 **Artifacts**:

| Artifact | 내용 |
|----------|------|
| `bundle-macos-arm64` | Apple Silicon용 `.app` |
| `bundle-macos-x64` | Intel Mac용 `.app` |
| `bundle-windows` | NSIS 설치 `.exe` |

압축을 풀어 Mac은 `.app`을 Applications에 복사, Windows는 설치 프로그램을 실행합니다.

### 로컬 빌드와의 차이

- Mac에서 `npm run tauri build` → 현재 Mac 아키텍처용 `.app`만 생성
- CI → **arm64 / x64 Mac + Windows** 한 번에 생성

## 외부 배포 참고

macOS/Windows에서 경고 없이 설치하려면 코드 서명 인증서가 필요합니다.
