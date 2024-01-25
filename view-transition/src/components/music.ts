import { createSignal } from "solid-js";

export const [music, setMusic] = createSignal<string | undefined>(undefined);
