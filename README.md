# DaevEngine

## 📌 프로젝트 소개

**DaevEngine**은 **네트워크 프로토콜 엔진**으로, 다양한 종류의 실시간 통신 및 데이터 전송을 위한 최적화된 프로토콜을 설계하고 구현하는 프로젝트입니다. 본 엔진은 **게임**, **스트리밍 서비스**, **파일 전송**, **채팅 시스템** 등 다양한 응용 분야에 사용할 수 있는 **TCP**와 **UDP** 기반의 맞춤형 네트워크 프로토콜을 제공합니다. 또한, 이를 통해 데이터 전송의 성능을 극대화하고, **패킷 손실 방지**, **저지연**, **연결 안정성**을 구현합니다.

**DaevEngine**은 게임 서버나 실시간 애플리케이션에서 발생할 수 있는 다양한 네트워크 문제를 해결하기 위해 설계되었습니다. 더 나아가, 여러 애플리케이션 환경에서 최적의 데이터 전송 방안을 연구하고 개발합니다.

## 🚀 사용 기술

- **Rust**: 고성능 네트워크 프로그래밍 및 성능 최적화를 위한 언어
- **TCP/UDP**: 두 가지 주요 프로토콜을 활용하여 **연결 지향적**(TCP) 및 **비연결형**(UDP) 데이터 전송 실험
- **실시간 게임 및 스트리밍**: 실시간 데이터 전송을 최적화하고, **패킷 손실 복구** 및 **저지연 처리** 기능 제공

## ⚙️ 주요 기능

- **TCP 기반 프로토콜**: 안정적이고 순서가 중요한 데이터 전송을 위한 **TCP** 프로토콜 사용
- **UDP 기반 프로토콜**: 빠르고 효율적인 데이터 전송을 위한 **UDP** 프로토콜 사용, 패킷 손실 및 지연 처리
- **게임 상태 전송**: 실시간 게임에서의 **상태 정보 전송**, **이벤트 처리** 및 **클라이언트 동기화**
- **스트리밍 최적화**: 비디오/오디오 스트리밍 서비스를 위한 최적의 프로토콜 설계 및 저지연 처리
- **패킷 손실 복구**: UDP에서 발생할 수 있는 패킷 손실을 복구하는 **효율적인 메커니즘** 연구
- **성능 최적화**: TCP/UDP 프로토콜의 성능을 실시간 애플리케이션에 맞게 최적화

## 📝 사용 방법

1. **Rust 환경 설정**: Rust가 설치되지 않았다면 [Rust 설치](https://www.rust-lang.org/) 가이드에 따라 설치합니다.
   
2. **프로젝트 클론**:
   ```bash
   git clone https://github.com/daev681/daevengine.git
   cd daevengine
```

전체구조흐름

sequenceDiagram
    participant Client
    participant Server (TCP Listener)
    participant AuthService
    participant SessionManager

    Client ->> Server (TCP Listener): Connect
    Client ->> Server (TCP Handler): Send AuthRequest (username, password)
    Server ->> AuthService: authenticate()
    AuthService -->> Server: AuthResponse (token)
    Server ->> SessionManager: insert(token, username)
    Server -->> Client: Send AuthResponse (success + token)

    loop 이후 통신
        Client ->> Server: Send packet with token
        Server ->> SessionManager: validate(token)
        alt token valid
            Server -->> Client: 정상 처리
        else token invalid
            Server -->> Client: 에러 응답 (Unauthorized)
        end
    end



DaevNetEngine
├─ Cargo.lock
├─ Cargo.toml
├─ README.md
└─ src
   ├─ auth
   │  ├─ auth_handler.rs
   │  ├─ auth_packet.rs
   │  ├─ README.md
   │  └─ token.rs
   ├─ config
   │  ├─ config.rs
   │  └─ mod.rs
   ├─ engine
   │  ├─ mod.rs
   │  └─ runtime.rs
   ├─ handler
   │  ├─ echo_handler.rs
   │  └─ mod.rs
   ├─ lib.rs
   ├─ main.rs
   ├─ net
   │  ├─ mod.rs
   │  ├─ packet.rs
   │  ├─ protocols
   │  │  ├─ mod.rs
   │  │  ├─ protobuf.rs
   │  │  ├─ protobuf_packet.rs
   │  │  ├─ protobuf_protocol.rs
   │  │  ├─ README.me
   │  │  ├─ xml_packet.rs
   │  │  └─ xml_protocol.rs
   │  ├─ tcp
   │  │  ├─ handler_tcp_connection.rs
   │  │  ├─ listener.rs
   │  │  └─ mod.rs
   │  └─ udp
   │     ├─ handle_udp_connection.rs
   │     └─ listener.rs
   └─ util
      ├─ buffer_pool.rs
      └─ mod.rs
