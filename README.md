# Matchmaking Server 🎮
Un service de matchmaking open-source en Rust, rapide et extensible. 📡

Ce projet permet d’appairer des joueurs en fonction de leur niveau et de leur latence, en utilisant Redis et une API REST/WebSockets.

## 📦 Installation
1. Cloner le repo
   ```bash
   git clone https://github.com/ton-utilisateur/matchmaking_server.git
   cd matchmaking_server
   ```
2. Installer Rust et Redis
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   sudo apt install redis
   ```
3. Lancer Redis
   ```bash
   redis-server
   ```
4. Lancer le serveur
   ```bash
   cargo run
   ```

## 🔥 Tester l'API

Envoyer un joueur en matchmaking:
```bash
curl -X POST "http://127.0.0.1:8080/queue" -H "Content-Type: application/json" -d '{"player_id":"player1", "skill":1500}'
```


## **💡 Fonctionnalités**
✅ File d'attente avec Redis  
✅ Algorithme de matchmaking simple (Elo)  
✅ API REST avec Actix Web  
🚧 WebSockets (en cours)  
🚧 Historique des matchs (à venir)  

Tu veux contribuer ? Regarde [CONTRIBUTING.md](CONTRIBUTING.md) et envoie une Pull Request ! 🚀

