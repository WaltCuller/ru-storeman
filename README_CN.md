## 进程管理

Rust process库new出的进程会和父进程共享一个pgid

Go os/exec库 创建进程会是独立的pgid