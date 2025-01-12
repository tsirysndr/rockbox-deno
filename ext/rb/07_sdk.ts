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
}

export abstract class RockboxExtension {
  constructor(public readonly metadata: ExtensionMetadata) {}

  abstract initialize(): void;
  abstract shutdown(): void;
  abstract getMetadata(): ExtensionMetadata;
}

export abstract class MusicProvider extends RockboxExtension {
  abstract browse(path?: string): Promise<Array<{ name: string; type: 'file' | 'directory'; path: string }>>;
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

  registerProvider(provider: MusicProvider) {
    provider.initialize();
    this.providers.push(provider);
  }

  registerStreamer(streamer: MediaStreamer) {
    streamer.initialize();
    this.streamers.push(streamer);
  }

  listProviders() {
    return this.providers.map((provider) => provider.metadata);
  }

  listStreamers() {
    return this.streamers.map((streamer) => streamer.metadata);
  }
}
