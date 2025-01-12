export type ExtensionMetadata = {
  name: string;
  version: string;
  author: string;
  license: string;
  readme?: string;
  tags?: string[];
  description?: string;
  repository?: string;
  homepage?: string;
  supportedUrls?: string[];
};

export abstract class RockboxExtension {
  constructor(public readonly metadata: ExtensionMetadata) {}

  abstract initialize(): void;
  abstract shutdown(): void;
  abstract getMetadata(): ExtensionMetadata;
}

export abstract class MusicProvider extends RockboxExtension {
  abstract browse(
    path?: string
  ): Promise<Array<{ name: string; type: "file" | "directory"; path: string }>>;
  abstract download(path: string): Promise<void>;
  abstract play(path: string): Promise<void>;
  abstract upload(path: string, destination: string): Promise<void>;
}

export abstract class MediaStreamer extends RockboxExtension {
  abstract stream(mediaUrl: string, target: string): Promise<void>;
  abstract stop(): Promise<void>;
  abstract pause(): Promise<void>;
  abstract resume(): Promise<void>;
  abstract seek(position: number): Promise<void>;
  abstract next(): Promise<void>;
  abstract previous(): Promise<void>;
  abstract listTargets(): Promise<
    Array<{
      name: string;
      type: string;
      address: string;
    }>
  >;
}

export class RockboxSDK {
  private providers: MusicProvider[] = [];
  private streamers: MediaStreamer[] = [];
  private extensionRegistry: Record<string, Record<string, Function>> = {};

  registerProvider(provider: MusicProvider) {
    provider.initialize();
    this.providers.push(provider);
    this.registerExtension(provider.metadata.name, provider);
  }

  registerStreamer(streamer: MediaStreamer) {
    streamer.initialize();
    this.streamers.push(streamer);
    this.registerExtension(streamer.metadata.name, streamer);
  }

  listProviders() {
    return this.providers.map((provider) => provider.metadata);
  }

  listStreamers() {
    return this.streamers.map((streamer) => streamer.metadata);
  }

  // deno-lint-ignore no-explicit-any
  private registerExtension(name: string, extension: any) {
    // Dynamically collect all methods of the extension
    const ignore = ["constructor", "initialize", "getMetadata"];
    const methods = Object.getOwnPropertyNames(Object.getPrototypeOf(extension))
      .filter(
        (method) =>
          !ignore.includes(method) && typeof extension[method] === "function"
      )
      .reduce((acc, method) => {
        acc[method] = extension[method].bind(extension);
        return acc;
      }, {} as Record<string, Function>);

    this.extensionRegistry[name] = methods;

    const ANSI_RESET = "\x1b[0m";
    const ANSI_CYAN = "\x1b[36m";
    const ANSI_GREEN = "\x1b[32m";
    const ANSI_YELLOW = "\x1b[33m";
    const ANSI_BG_GREEN = "\x1b[42m";

    // Use ANSI codes directly in strings
    console.log(`\n${ANSI_CYAN}[INFO] Extension Registered${ANSI_RESET}`);
    console.log(
      "------------------------------------------------------------------------------"
    );
    console.log(
      `${ANSI_GREEN}Name       :${ANSI_RESET} ${ANSI_BG_GREEN}${name}${ANSI_RESET}`
    );
    console.log(
      `${ANSI_GREEN}Methods    :${ANSI_RESET} ${ANSI_YELLOW}${Object.keys(
        methods
      ).join(", ")}${ANSI_RESET}`
    );
    console.log(
      `${ANSI_GREEN}Registered :${ANSI_RESET} ${new Date().toISOString()}`
    );
    console.log(
      "------------------------------------------------------------------------------"
    );
  }

  getExtensionRegistry(): Record<string, Record<string, Function>> {
    return this.extensionRegistry;
  }
}
