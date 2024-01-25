import { createSignal } from "solid-js";
import { createStore } from "solid-js/store";

export function SSE() {
  let update = createStream();
  return (
    <div>
      <h1>Server Sent Events</h1>
      <h2>Message: {update()}</h2>
      <h2>Stream Status: {update.state}</h2>
      <div>
        <button onClick={update.startRequest}>Start Request</button>
      </div>
    </div>
  );
}

type Stream = {
  (): string | undefined;
  state: number;
  startRequest: () => void;
};

function createStream(): Stream {
  let [update, setUpdate] = createSignal<string | undefined>(undefined);
  let [state, setState] = createStore({
    state: "idle" as "idle" | "connecting" | "open" | "closed",
    startRequest: () => {
      if (state.state === "open") return;
      const evtSource = new EventSource("http://localhost:3000/stream");
      setState({ state: "connecting" });
      evtSource.onopen = () => {
        setState({ state: "open" });
      };
      evtSource.onmessage = (event) => {
        setUpdate(event.data);
      };
      evtSource.onerror = () => {
        setState({ state: "closed" });
        evtSource.close();
      };
      setState({ state: "connecting" });
    },
  });
  let getUpdate = () => update();
  Object.setPrototypeOf(getUpdate, state); //can't be picked up typescript
  if (import.meta.env.SSR) return getUpdate as Stream;
  return getUpdate as Stream;
}
