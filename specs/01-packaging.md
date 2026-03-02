# 미꾸라지 DNS 패키징 및 배포 전략

미꾸라지 DNS는 3개의 별도 프로세스(Core, UI, Service)로 구성되며,
데스크톱(Windows, macOS, Linux) 및 모바일(iOS, Android) 환경을 모두 지원해야 합니다.
따라서 각 OS의 특성, 샌드박스 정책, 그리고 권한 관리 방식에 맞춘 차별화된 패키징 및 배포 전략이
필요합니다.

## 1. 데스크톱 환경 (Windows, macOS, Linux)

데스크톱 환경에서는 Core, UI, Service 3개의 실행 파일(또는 바이너리)이 배포되며,
OS별 서비스 관리자(Service Manager)에 **Service Process**를 등록하는
과정이 설치(Install) 단계에 포함되어야 합니다.

### Windows

* **패키징 포맷:** MSI (WiX Toolset 권장)

* **구성 요소 및 권한:**
  * `mudfish-dns-ui.exe`: 사용자 권한으로 실행되는 UI 애플리케이션
    (시작 메뉴 및 시스템 트레이 통합).
  * `mudfish-dns-service.exe`: Windows Service로 등록되어 백그라운드에서
    시스템 권한(`NT AUTHORITY\SYSTEM`)으로 자동 실행됩니다.
  * `mudfish-dns-core.exe`: Service 프로세스에 의해 관리(Watchdog)되며, WFP나 Wintun을
    제어하기 위해 관리자 권한으로 실행됩니다.
  
* **설치 특이사항:**
  * 인스톨러 실행 시 UAC(사용자 계정 컨트롤) 권한 상승을 요청하여 `sc create` 명령어로 서비스를
    등록해야 합니다.
  * Wintun 드라이버 등 네트워크 캡처에 필요한 파일(.sys, .dll)을 함께 패키징해야 합니다.
  * Windows SmartScreen 경고를 방지하기 위해 **EV Code Signing (확장 유효성 코드 서명)**
    필수적입니다.

### macOS

* **패키징 및 배포:** Mac App Store 전용 배포
  * macOS 환경에서는 오직 **Mac App Store**를 통해서만 배포합니다.

* **구성 요소 및 권한:**
  * App Store 정책(App Sandbox)을 준수하기 위해 `launchd`를 통한 글로벌 데몬 방식은
    사용할 수 없으며, 단일 앱 번들로 구성해야 합니다.
  * macOS의 공식 **`NetworkExtension` (DNSProxyProvider 등)**을 사용하여 UI 앱 번들
    내에 System Extension / Network Extension 형태로 네트워크 처리 로직
    (Core Process 연동)을 포함시킵니다.
  * OS가 자체적으로 사용자에게 네트워크 확장 프로파일 설치 및 권한 허용 팝업을 띄우며, 권한
    관리를 대행합니다.
  
* **설치 특이사항:**
  * Apple Developer 계정을 통한 Mac App Store 배포용 서명(App Store Distribution)
    및 프로비저닝 프로파일 관리가 필수적입니다.
  * App Store 심사 가이드라인을 준수하도록 개발 및 패키징되어야 합니다.

### Linux

* **패키징 포맷:** DEB (Debian/Ubuntu 계열), RPM (RHEL/CentOS/Fedora 계열)

* **구성 요소 및 권한:**
  * 바이너리 배포: 바이너리 파일들은 `/opt/mudfish-dns/bin/` 에 위치합니다.
  * 데스크톱 통합: `.desktop` 파일을 `/usr/share/applications/`에 배포하여 애플리케이션
    메뉴에 노출합니다.
  * 데몬 등록: `systemd` 서비스 유닛 파일(`mudfish-dns.service`)을
    `/etc/systemd/system/`에 배포하고 설치 스크립트(post-install)를 통해
    활성화(`systemctl enable --now`)합니다.
  
* **설치 특이사항:**
  * 패키지 매니저를 통해 설치되므로 root 권한 스크립트 실행이 용이합니다.
  * eBPF 기반 패킷 캡처를 사용할 경우, `mudfish-dns-core` 바이너리에
    `CAP_NET_ADMIN`, `CAP_BPF` 등의 Linux Capability를 부여하는 과정이 설치 시 포함될
    수 있습니다.

---

## 2. 모바일 환경 (iOS, Android)

모바일 환경은 데스크톱의 멀티 프로세스(Core/UI/Service) 구조를 그대로 유지할 수 없습니다.
UI는 네이티브 언어나 크로스 플랫폼 프레임워크로 작성되고,
**Core Process는 라이브러리(Library) 형태로 임베딩**되어야 합니다. Service Process의
역할은 모바일 OS가 제공하는 고유의 VPN API로 대체됩니다.

### 공통 빌드 전략

* `mudfish-dns-core` 코드를 라이브러리(`cdylib` 또는 `staticlib`)로 빌드하고
  FFI(Foreign Function Interface)를 통해 노출합니다.
* [Mozilla UniFFI](https://github.com/mozilla/uniffi-rs)와 같은 도구를 사용하면
  Rust 구조체를 Kotlin이나 Swift용 바인딩 코드로 자동 생성할 수 있어 효율적입니다.

### Android

* **패키징 포맷:** APK / AAB (Google Play Store 배포)

* **통합 방식:**
  * `core`를 Android JNI를 통해 로드할 수 있는 네이티브
    라이브러리(`libmudfish_dns_core.so`)로 크로스 컴파일(NDK 활용)합니다.
  * Android의 `VpnService` 클래스를 상속받는 서비스를 앱 내에 구현하여 기기 전체의
    트래픽/DNS를 가로채고, 해당 파일 디스크립터(File Descriptor)를 Rust Core에 넘겨
    처리하도록 구성합니다.

### iOS

* **패키징 포맷:** IPA (Apple App Store 배포)

* **통합 방식:**
  * `core`를 `XCFramework` 형태로 컴파일하여 Xcode 프로젝트에 링킹합니다.
  * iOS의 `NetworkExtension` (특히 `NEDNSSettingsManager` 또는
    `NEPacketTunnelProvider`)을 App Extension 타겟으로 추가하고, 이 확장 프로세스
    내부에서 Rust Core 라이브러리를 초기화하여 DNS 요청을 가로채고 처리합니다.

---

## 3. 배포 및 자동화 (CI/CD)

효율적인 멀티 플랫폼 배포를 위해 GitHub Actions 등 CI/CD 파이프라인을 구축합니다.

* **바이너리 빌드 및 패키징 (Matrix Build):**
  * Windows: `cargo build` 후 `cargo-wix`를 사용하여 `.msi` 생성 및 EV 인증서 서명.
  * macOS: `cargo build` 후 `.app` 번들 생성, `xcodebuild` 등을 활용하여 App Store 배포용 아카이브 생성 및 App Store Connect 업로드.
  * Linux: `cargo-deb`, `cargo-generate-rpm`을 사용하여 배포판별 패키지 자동 생성.

* **모바일 라이브러리 빌드:**
  * Android: `cargo-ndk`를 사용하여 아키텍처별 `.so` 파일 빌드.
  * iOS: `cargo-lipo` 또는 xcodebuild 스크립트를 통한 `.xcframework` 구성.

* **업데이트 시스템 배포:**
  * CI 환경에서 릴리즈 태그 생성 시, 바이너리뿐만 아니라 버전 정보, 체크섬(SHA256)이 포함된 `update-metadata.json`을 생성하여 S3 등 정적 서버에 배포합니다. 데스크톱 UI 앱은 이 메타데이터를 폴링하여 인앱 자동 업데이트(In-app Auto Update) 기능을 제공합니다.
