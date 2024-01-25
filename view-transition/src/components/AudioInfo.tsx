import { setMusic } from "./music";

export function AudioInfo(props: AudioInfoProps) {
  function changeMusic() {
    setMusic(props.uri);
  }
  return (
    <div class="p-4 rounded bg-gray-50">
      <div>{props.description}</div>
      <div class="flex justify-center gap-4">
        <button
          onclick={() => history.back()}
          class="rounded w-fit bg-gray-300 px-4 py-0.5"
        >
          Back
        </button>
        <button
          onclick={changeMusic}
          class="rounded w-fit bg-green-300 px-4 py-0.5"
        >
          Play
        </button>
      </div>
    </div>
  );
}

export type AudioInfoProps = {
  description: string;
  uri: string;
};
