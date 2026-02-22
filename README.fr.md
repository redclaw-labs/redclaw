<p align="center">
  <img src="logo.png" alt="RedClaw" width="200" />
</p>

<h1 align="center">RedClaw 🦀（Français）</h1>

<p align="center">
  <strong>Zéro overhead. Zéro compromis. 100% Rust. 100% agnostique.</strong>
</p>

<p align="center">
  <a href="https://x.com/redclawlabs?s=21"><img src="https://img.shields.io/badge/X-%40redclawlabs-000000?style=flat&logo=x&logoColor=white" alt="X: @redclawlabs" /></a>
  <a href="https://www.xiaohongshu.com/user/profile/67cbfc43000000000d008307?xsec_token=AB73VnYnGNx5y36EtnnZfGmAmS-6Wzv8WMuGpfwfkg6Yc%3D&xsec_source=pc_search"><img src="https://img.shields.io/badge/Xiaohongshu-Official-FF2442?style=flat" alt="Xiaohongshu: Official" /></a>
  <a href="https://t.me/redclawlabs"><img src="https://img.shields.io/badge/Telegram-%40redclawlabs-26A5E4?style=flat&logo=telegram&logoColor=white" alt="Telegram: @redclawlabs" /></a>
  <a href="https://t.me/redclawlabs_cn"><img src="https://img.shields.io/badge/Telegram%20CN-%40redclawlabs__cn-26A5E4?style=flat&logo=telegram&logoColor=white" alt="Telegram CN: @redclawlabs_cn" /></a>
  <a href="https://t.me/redclawlabs_ru"><img src="https://img.shields.io/badge/Telegram%20RU-%40redclawlabs__ru-26A5E4?style=flat&logo=telegram&logoColor=white" alt="Telegram RU: @redclawlabs_ru" /></a>
  <a href="https://www.reddit.com/r/redclawlabs/"><img src="https://img.shields.io/badge/Reddit-r%2Fredclawlabs-FF4500?style=flat&logo=reddit&logoColor=white" alt="Reddit: r/redclawlabs" /></a>
</p>

<p align="center">
  🌐 Langues : <a href="README.md">English</a> · <a href="README.zh-CN.md">简体中文</a> · <a href="README.ja.md">日本語</a> · <a href="README.ru.md">Русский</a> · <a href="README.vi.md">Tiếng Việt</a> · Français
</p>

<p align="center">
  <a href="bootstrap.sh">Bootstrap en 1 clic</a> |
  <a href="docs/getting-started/README.md">Guide de démarrage</a> |
  <a href="docs/README.fr.md">Hub de documentation</a> |
  <a href="docs/SUMMARY.md">TOC docs</a>
</p>

<p align="center">
  <strong>Routage rapide :</strong>
  <a href="docs/reference/README.md">Référence</a> ·
  <a href="docs/operations/README.md">Opérations & déploiement</a> ·
  <a href="docs/troubleshooting.md">Dépannage</a> ·
  <a href="docs/security/README.md">Sécurité</a> ·
  <a href="docs/hardware/README.md">Matériel</a> ·
  <a href="docs/contributing/README.md">Contribution & CI</a>
</p>

> Ce document est une traduction alignée manuellement de `README.md`, avec une priorité sur la précision et la lisibilité (ce n’est pas une traduction mot à mot).
>
> Les identifiants techniques (commandes, clés de configuration, chemins d’API, noms de Trait, etc.) restent en anglais afin d’éviter toute dérive sémantique.
>
> Dernière synchronisation : **2026-02-22**.

## 📢 Tableau d’annonces

Cette section est utilisée pour les avis importants (breaking changes, annonces de sécurité, fenêtres de maintenance, blocages de release, etc.).

| Date (UTC) | Niveau | Annonce | Action recommandée |
|---|---|---|---|
| 2026-02-19 | _Urgent_ | Nous n’avons **aucun lien** avec `openagen/redclaw` ni avec `redclaw.org`. `redclaw.org` pointe actuellement vers le fork `openagen/redclaw`, et ce domaine/dépôt usurpe l’identité de notre site et de notre projet officiels. | Ne faites pas confiance aux informations, binaires, collectes de fonds, ou « annonces officielles » provenant de ces sources. Fiez-vous uniquement à `github.com/redclaw-labs/redclaw` et aux comptes sociaux vérifiés liés dans les badges ci-dessus. |
| 2026-02-19 | _Important_ | Nous n’avons **pas encore de site officiel**, et nous avons constaté des tentatives d’usurpation. Ne participez pas à des activités d’investissement ou de collecte de fonds au nom de RedClaw. | Vérifiez tout d’abord dans ce dépôt ; suivez [X (@redclawlabs)](https://x.com/redclawlabs?s=21), [Reddit (r/redclawlabs)](https://www.reddit.com/r/redclawlabs/), [Telegram (@redclawlabs)](https://t.me/redclawlabs), [Telegram CN (@redclawlabs_cn)](https://t.me/redclawlabs_cn), [Telegram RU (@redclawlabs_ru)](https://t.me/redclawlabs_ru) et le [compte Xiaohongshu](https://www.xiaohongshu.com/user/profile/67cbfc43000000000d008307?xsec_token=AB73VnYnGNx5y36EtnnZfGmAmS-6Wzv8WMuGpfwfkg6Yc%3D&xsec_source=pc_search) pour les mises à jour officielles. |
| 2026-02-19 | _Important_ | Anthropic a mis à jour « Authentication and Credential Use » le 2026-02-19. Les termes précisent que l’OAuth authentication (Free/Pro/Max) est réservé à Claude Code et Claude.ai ; l’utilisation de tokens OAuth issus de Claude Free/Pro/Max dans d’autres produits/outils/services (y compris des Agent SDK) n’est pas autorisée et peut constituer une violation des Consumer Terms of Service. | Pour réduire le risque, n’essayez pas l’intégration OAuth Claude Code pour le moment. Source : [Authentication and Credential Use](https://code.claude.com/docs/en/legal-and-compliance#authentication-and-credential-use). |

## Aperçu du projet

RedClaw est un runtime d’agent autonome optimisé pour la performance, l’efficacité en ressources et l’extensibilité.

- Implémentation Rust native, distribuable en binaire unique
- Architecture basée sur des Trait (`Provider` / `Channel` / `Tool` / `Memory`…)
- Sécurisé par défaut (pairing, allowlist explicite, sandbox, contrôle de scope)

## Pourquoi RedClaw ?

- **Standardiser un runtime léger** : les opérations courantes (CLI, `status`, etc.) tournent avec quelques MB de RAM.
- **Adapté aux environnements à faible coût** : fonctionne sur des cartes peu chères ou de petits serveurs, sans grosse « plateforme » d’exécution.
- **Cold start très rapide** : le binaire Rust unique démarre très vite, y compris pour les commandes principales et le daemon.
- **Portabilité élevée** : ARM / x86 / RISC-V avec le même modèle d’exploitation ; providers/channels/tools sont interchangeables.

## Instantané de benchmark (RedClaw vs OpenClaw, reproductible)

Ci-dessous une comparaison rapide locale (macOS arm64, février 2026), normalisée sur une base CPU edge à 0,8 GHz.

| | OpenClaw | NanoBot | PicoClaw | RedClaw 🦀 |
|---|---|---|---|---|
| **Langage** | TypeScript | Python | Go | **Rust** |
| **RAM** | > 1GB | > 100MB | < 10MB | **< 5MB** |
| **Temps de démarrage (cœur 0,8GHz)** | > 500s | > 30s | < 1s | **< 10ms** |
| **Taille binaire** | ~28MB (dist) | N/A (script) | ~8MB | **3.4 MB** |
| **Coût** | Mac Mini $599 | Linux SBC ~$50 | Linux board $10 | **n’importe quel matériel à $10** |

> Note : les résultats RedClaw ont été mesurés en build release via `/usr/bin/time -l`. OpenClaw requiert un runtime Node.js et ajoute typiquement ~390MB de RAM uniquement pour ce runtime. NanoBot requiert un runtime Python. PicoClaw et RedClaw sont des binaires statiques.

<p align="center">
  <img src="benchmark.jpeg" alt="RedClaw vs OpenClaw Comparison" width="800" />
</p>

### Mesure reproductible en local

Les chiffres évoluent avec le code et la toolchain ; re-mesurez dans votre environnement cible.

```bash
cargo build --release
ls -lh target/release/redclaw

/usr/bin/time -l target/release/redclaw --help
/usr/bin/time -l target/release/redclaw status
```

Exemple dans ce README (macOS arm64, 2026-02-18) :

- Binaire release : `8.8M`
- `redclaw --help` : ~`0.02s`, pic RAM ~`3.9MB`
- `redclaw status` : ~`0.01s`, pic RAM ~`4.1MB`

## Bootstrap en un clic

```bash
git clone https://github.com/redclaw-labs/redclaw.git
cd redclaw
./bootstrap.sh
```

Initialiser aussi l’environnement : `./bootstrap.sh --install-system-deps --install-rust` (peut nécessiter `sudo`).

Détails : [`docs/one-click-bootstrap.md`](docs/one-click-bootstrap.md).

## Fonctionnalités

<a name="providers"></a>

### 🤖 30+ providers de modèles

RedClaw inclut des providers first-party ainsi que des adaptateurs et alias OpenAI-compatible.

Points forts :
- OpenAI (`openai`)
- Anthropic (`anthropic`)
- Google Gemini (`gemini` / `google`)
- OpenRouter (`openrouter`)
- Ollama (`ollama`)
- Groq, Mistral, xAI, DeepSeek, Together, Fireworks, Perplexity, Cohere
- Amazon Bedrock (`bedrock` / `aws-bedrock`)
- Qwen / DashScope, GLM / Zhipu, Moonshot / Kimi, MiniMax
- NVIDIA NIM (`nvidia` / `nvidia-nim`)

Endpoints custom :
- `custom:https://your-api.com` (OpenAI-compatible)
- `anthropic-custom:https://your-api.com` (Anthropic-compatible)

<a name="channels"></a>

### 📡 16 channels de communication

- CLI
- Telegram
- Discord
- Slack
- WhatsApp
- Matrix
- IRC
- Linq
- Email
- Signal
- Mattermost
- Nextcloud Talk
- DingTalk
- Lark
- Webhook
- QQ

<a name="tools"></a>

### 🛠️ 22 outils intégrés

Outils clés livrés in-tree :
- Exécution shell (runtime natif + politiques sandbox)
- Lecture/écriture de fichiers (scopé au workspace)
- Stockage/rappel/forget de mémoire
- Planification / scheduling
- Opérations Git
- Requêtes HTTP (allowlist de domaines)
- Ouverture de navigateur + automation (optionnel)
- Capture d’écran + inspection d’image
- Helpers hardware (info board + memory map/read)
- Outil de délégation (optionnel, quand des agents supplémentaires sont configurés)

## Démarrage rapide

### Homebrew (macOS/Linuxbrew)

```bash
brew install redclaw
```

```bash
git clone https://github.com/redclaw-labs/redclaw.git
cd redclaw
cargo build --release --locked
cargo install --path . --force --locked

redclaw onboard --api-key sk-... --provider openrouter
redclaw onboard --interactive

redclaw agent -m "Hello, RedClaw!"

# default: 127.0.0.1:3000
redclaw gateway

redclaw daemon
```

## Authentification par abonnement (OpenAI Codex / Claude Code)

RedClaw prend en charge des profils d’authentification natifs “type abonnement” (multi-comptes, stockage chiffré).

- Config file: `~/.redclaw/auth-profiles.json`
- Encryption key: `~/.redclaw/.secret_key`
- Profile ID format: `<provider>:<profile_name>` (example: `openai-codex:work`)

OpenAI Codex OAuth (ChatGPT subscription):

```bash
# Recommended for server/headless environments
redclaw auth login --provider openai-codex --device-code

# Browser/callback flow (with paste fallback)
redclaw auth login --provider openai-codex --profile default
redclaw auth paste-redirect --provider openai-codex --profile default

# Check / refresh / switch profiles
redclaw auth status
redclaw auth refresh --provider openai-codex --profile default
redclaw auth use --provider openai-codex --profile work
```

Claude Code / Anthropic setup-token:

```bash
# Paste subscription/setup token (Authorization header mode)
redclaw auth paste-token --provider anthropic --profile default --auth-kind authorization

# Alias
redclaw auth setup-token --provider anthropic --profile default
```

Run the agent with subscription auth:

```bash
redclaw agent --provider openai-codex -m "hello"
redclaw agent --provider openai-codex --auth-profile openai-codex:work -m "hello"

# Anthropic supports API key and auth token environment variables:
# ANTHROPIC_AUTH_TOKEN, ANTHROPIC_OAUTH_TOKEN, ANTHROPIC_API_KEY
redclaw agent --provider anthropic -m "hello"
```

## Architecture

Tous les sous-systèmes sont des **Trait** — vous pouvez remplacer les implémentations via la configuration, sans réécrire l’agent.

<p align="center">
  <img src="docs/architecture.svg" alt="Architecture RedClaw" width="900" />
</p>

| Sous-système | Trait | Implémentations intégrées | Extension |
|-------------|-------|----------|----------|
| **Modèle IA** | `Provider` | Voir `redclaw providers` (providers intégrés + alias, endpoints custom) | `custom:https://your-api.com` (OpenAI-compatible) ou `anthropic-custom:https://your-api.com` |
| **Channels** | `Channel` | CLI, Telegram, Discord, Slack, Mattermost, Linq, Matrix, Signal, WhatsApp, Email, IRC, Lark, DingTalk, QQ | N’importe quelle API de messagerie |
|  |  | Webhook, Nextcloud Talk |  |
| **Mémoire** | `Memory` | Recherche hybride SQLite, backend PostgreSQL, pont Lucid, fichiers Markdown, backend explicite `none`, snapshot/hydrate, cache de réponse optionnel | N’importe quel backend de persistance |
| **Outils** | `Tool` | 22 outils : shell/file/memory, cron/schedule, git, pushover, browser, http_request, screenshot/image_info, composio (opt-in), delegate, outils hardware | N’importe quelle capacité |
| **Observabilité** | `Observer` | Noop, Log, Multi | Prometheus, OTel |
| **Runtime** | `RuntimeAdapter` | Native, Docker (sandbox) | Ajout via adapter ; les kinds non supportés échouent explicitement |
| **Sécurité** | `SecurityPolicy` | Pairing gateway, sandbox, allowlists, rate limits, scope filesystem, secrets chiffrés | — |
| **Identité** | `IdentityConfig` | OpenClaw (markdown), AIEOS v1.1 (JSON) | N’importe quel format d’identité |
| **Tunnel** | `Tunnel` | None, Cloudflare, Tailscale, ngrok, Custom | N’importe quel outil de tunnel |
| **Heartbeat** | Engine | Tâches périodiques HEARTBEAT.md | — |
| **Compétences** | Loader | Manifeste TOML + instruction SKILL.md | Packs de compétences communautaires |
| **Intégrations** | Registry | 9 catégories, 70+ intégrations | Système de plugins |

### Support runtime (actuel)

- ✅ Supporté : `runtime.kind = "native"` ou `runtime.kind = "docker"`
- 🚧 Planifié (non implémenté) : WASM / edge runtime

Si vous configurez un `runtime.kind` non supporté, RedClaw s’arrête avec une erreur explicite plutôt que de faire un fallback silencieux.

### Système de mémoire (moteur de recherche full-stack)

Implémentation 100% in-tree, zéro service externe — pas besoin de Pinecone, Elasticsearch, ni LangChain :

| Couche | Implémentation |
|---------|------|
| **Vector DB** | Embeddings stockés en BLOB dans SQLite, recherche par similarité cosinus |
| **Recherche par mots-clés** | Tables virtuelles FTS5, scoring BM25 |
| **Fusion hybride** | Fonction de fusion pondérée personnalisée (`vector.rs`) |
| **Embeddings** | Trait `EmbeddingProvider` — OpenAI, URL custom, ou noop |
| **Chunking** | Chunker Markdown ligne-par-ligne conservant la structure des titres |
| **Cache** | Table SQLite `embedding_cache`, politique LRU |
| **Réindexation sûre** | Rebuild FTS5 atomique + re-embed des vecteurs manquants |

L’agent rappelle/enregistre/gère la mémoire automatiquement via des outils.

```toml
[memory]
backend = "sqlite"             # "sqlite", "lucid", "postgres", "markdown", "none"
auto_save = true
embedding_provider = "none"    # "none", "openai", "custom:https://..."
vector_weight = 0.7
keyword_weight = 0.3
```

## Paramètres de sécurité par défaut

- Bind gateway par défaut : `127.0.0.1:3000`
- Pairing gateway par défaut : `require_pairing = true`
- Refus par défaut du bind public : `allow_public_bind = false`
- Semantique d’allowlist des channels :
  - liste vide `[]` => deny-by-default
  - `"*"` => allow all (uniquement si vous comprenez pleinement le risque)

## Extrait de configuration (exemple)

```toml
api_key = "sk-..."
default_provider = "openrouter"
default_model = "anthropic/claude-sonnet-4.6"
default_temperature = 0.7

[memory]
backend = "sqlite"             # sqlite | lucid | postgres | markdown | none
auto_save = true
embedding_provider = "none"    # none | openai | custom:https://...

[gateway]
host = "127.0.0.1"
port = 3000
require_pairing = true
allow_public_bind = false
```

## Points d’entrée documentation

- Hub de documentation (anglais) : [`docs/README.md`](docs/README.md)
- Table des matières unifiée (TOC) : [`docs/SUMMARY.md`](docs/SUMMARY.md)
- Hub de documentation (français) : [`docs/README.fr.md`](docs/README.fr.md)
- Référence des commandes : [`docs/commands-reference.md`](docs/commands-reference.md)
- Référence configuration : [`docs/config-reference.md`](docs/config-reference.md)
- Référence providers : [`docs/providers-reference.md`](docs/providers-reference.md)
- Référence channels : [`docs/channels-reference.md`](docs/channels-reference.md)
- Runbook opérations : [`docs/operations-runbook.md`](docs/operations-runbook.md)
- Dépannage : [`docs/troubleshooting.md`](docs/troubleshooting.md)
- Inventaire / classification docs : [`docs/docs-inventory.md`](docs/docs-inventory.md)
- Instantané triage projet : [`docs/project-triage-snapshot-2026-02-18.md`](docs/project-triage-snapshot-2026-02-18.md)

## Contribution / licence

- Contribution : [`CONTRIBUTING.md`](CONTRIBUTING.md)
- Workflow PR : [`docs/pr-workflow.md`](docs/pr-workflow.md)
- Guide reviewer : [`docs/reviewer-playbook.md`](docs/reviewer-playbook.md)
- Licence : MIT ([`LICENSE`](LICENSE), [`NOTICE`](NOTICE))

---

Pour les spécifications complètes (toutes les commandes, l’architecture, les API et les flux de développement), reportez-vous à la version anglaise : [`README.md`](README.md).
