import { spawn } from "node:child_process";
import { text } from "node:stream/consumers";

export const LSP_BINARY_NAME = "toml-lsp";

export class Server {
  private serverVersion?: string;

  constructor(private serverPath: string) {}

  async showVersion(): Promise<string> {
    if (this.serverVersion === undefined) {
      let version: string;
      try {
        version = await text(
          spawn(this.serverPath, ["--version"]).stdout.setEncoding("utf-8"),
        );
        // version の先頭文字が LSP_BINARY_NAME で始まる場合は、その文字列を削除し、文字列の先頭の空白も削除する
        if (version.startsWith(LSP_BINARY_NAME)) {
          version = version.slice(LSP_BINARY_NAME.length).trimStart();
        }
      } catch {
        version = "<unknown>";
      }

      this.serverVersion = version;

      return version;
    }

    return this.serverVersion;
  }
}
