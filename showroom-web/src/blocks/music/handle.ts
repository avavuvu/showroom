import { context } from "../../../bindings/boutique/setups";
import type { MusicAttrs } from "./attrs";

export interface MusicHandle {
    attrs(): MusicAttrs;
    status(): string;
    busy(): boolean;
    set(attrs: MusicAttrs): void;
    remove(): void;
    addLink(text: string): Promise<boolean>;
    pickCover(): void;
    focusEditor(): void;
    onUpdate(listener: () => void, signal: AbortSignal): void;
}

export const musicNode = context<MusicHandle>("music-node");
