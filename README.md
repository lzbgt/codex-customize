[![CodexW_Setup_Support](https://img.shields.io/badge/CodexW_Setup_Support-brightgreen)](https://x2.brucelu.top/codexw/checkout/?source=github-badge-codex-customize) [![Ask_First](https://img.shields.io/badge/Ask_First-blue)](https://x2.brucelu.top/products/contact/?offer=codexw&source=github-badge-codex-customize) [![Sample](https://img.shields.io/badge/Sample-informational)](https://x2.brucelu.top/codexw/sample/)

> Unofficial local workflow support note: this repository is not OpenAI official support. If you want help setting up a private local Codex terminal workflow, use the CodexW setup path below.

## Paid CodexW setup support

Using this repo to customize local Codex CLI behavior, prompts, AGENTS.md, wrappers, or continuation policy? CodexW Founding Access provides focused setup/onboarding support:

- Ask a pre-sales question: https://x2.brucelu.top/products/contact/?offer=codexw&source=github-codex-customize-top
- Sample setup runbook: https://x2.brucelu.top/codexw/sample/
- Checkout: https://x2.brucelu.top/codexw/checkout/?source=github-codex-customize-top

Boundary: paid support is setup/onboarding guidance for local Codex terminal workflows. It does not include OpenAI API credits, guaranteed Codex availability, official OpenAI support, credential-managed services, or changes to OpenAI products.

---

<p align="center"><code>npm i -g @openai/codex</code><br />or <code>brew install --cask codex</code></p>
<p align="center"><strong>Codex CLI</strong> is a coding agent from OpenAI that runs locally on your computer.
<p align="center">
  <img src="./.github/codex-cli-splash.png" alt="Codex CLI splash" width="80%" />
</p>
</br>
If you want Codex in your code editor (VS Code, Cursor, Windsurf), <a href="https://developers.openai.com/codex/ide">install in your IDE.</a>
</br>If you are looking for the <em>cloud-based agent</em> from OpenAI, <strong>Codex Web</strong>, go to <a href="https://chatgpt.com/codex">chatgpt.com/codex</a>.</p>

---

## Quickstart

### Installing and running Codex CLI

Install globally with your preferred package manager:

```shell
# Install using npm
npm install -g @openai/codex
```

```shell
# Install using Homebrew
brew install --cask codex
```

Then simply run `codex` to get started.

<details>
<summary>You can also go to the <a href="https://github.com/openai/codex/releases/latest">latest GitHub Release</a> and download the appropriate binary for your platform.</summary>

Each GitHub Release contains many executables, but in practice, you likely want one of these:

- macOS
  - Apple Silicon/arm64: `codex-aarch64-apple-darwin.tar.gz`
  - x86_64 (older Mac hardware): `codex-x86_64-apple-darwin.tar.gz`
- Linux
  - x86_64: `codex-x86_64-unknown-linux-musl.tar.gz`
  - arm64: `codex-aarch64-unknown-linux-musl.tar.gz`

Each archive contains a single entry with the platform baked into the name (e.g., `codex-x86_64-unknown-linux-musl`), so you likely want to rename it to `codex` after extracting it.

</details>

### Using Codex with your ChatGPT plan

Run `codex` and select **Sign in with ChatGPT**. We recommend signing into your ChatGPT account to use Codex as part of your Plus, Pro, Team, Edu, or Enterprise plan. [Learn more about what's included in your ChatGPT plan](https://help.openai.com/en/articles/11369540-codex-in-chatgpt).

You can also use Codex with an API key, but this requires [additional setup](https://developers.openai.com/codex/auth#sign-in-with-an-api-key).

## Docs

- [**Codex Documentation**](https://developers.openai.com/codex)
- [**Contributing**](./docs/contributing.md)
- [**Installing & building**](./docs/install.md)
- [**Open source fund**](./docs/open-source-fund.md)

This repository is licensed under the [Apache-2.0 License](LICENSE).
