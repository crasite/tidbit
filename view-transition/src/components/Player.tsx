import { createEffect, createSignal } from "solid-js";
import { music, setMusic } from "./music";

export function Player() {
  const [isPlaying, setIsPlaying] = createSignal(false);
  const [audio] = createSignal(new Audio(music() ?? ""));
  createEffect(async () => {
    let src = music();
    if (src && audio().src !== src) {
      audio().src = src;
      audio().load();
      try {
        await audio().play();
        setIsPlaying(true);
      } catch (e) {
        setMusic(undefined);
      }
    }
  });

  function play() {
    if (!audio().src.endsWith(".mp3")) return;
    const playing = isPlaying();
    setIsPlaying(!playing);
    if (playing) {
      audio().pause();
    } else {
      audio().play();
    }
  }
  return (
    <div class="rounded p-2 bg-blue-50">
      <div classList={{ "animate-spin": isPlaying() }} onclick={play}>
        <Disk />
      </div>
    </div>
  );
}

function Disk(props: { class?: string }) {
  return (
    <svg
      //@ts-ignore
      xmlns:dc="http://purl.org/dc/elements/1.1/"
      xmlns:cc="http://creativecommons.org/ns#"
      xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
      xmlns:svg="http://www.w3.org/2000/svg"
      xmlns="http://www.w3.org/2000/svg"
      xmlns:xlink="http://www.w3.org/1999/xlink"
      viewBox="0 0 400 400"
      id="svg2"
      version="1.0"
      class={props.class}
    >
      <defs id="defs4">
        <linearGradient id="linearGradient3273">
          <stop style="stop-color:#6c6c6c" offset="0" id="stop3275" />
          <stop style="stop-color:#3e3e3e" offset="1" id="stop3277" />
        </linearGradient>
        <linearGradient id="linearGradient3263">
          <stop style="stop-color:#f7f7f7" offset="0" id="stop3265" />
          <stop style="stop-color:#e1e1e1" offset="1" id="stop3267" />
        </linearGradient>
        <linearGradient id="linearGradient3153">
          <stop style="stop-color:#ffffff" offset="0" id="stop3155" />
          <stop
            style="stop-color:#ffffff;stop-opacity:0"
            offset="1"
            id="stop3157"
          />
        </linearGradient>
        <filter id="filter3315">
          <feGaussianBlur stdDeviation="3.7" id="feGaussianBlur3317" />
        </filter>
        <radialGradient
          //@ts-ignore
          xlink:href="#linearGradient3153"
          id="radialGradient3328"
          gradientUnits="userSpaceOnUse"
          gradientTransform="matrix(-0.8695893,0.8695893,-1.9614862,-1.9614862,681.5104,178.1413)"
          cx="289.42923"
          cy="209.66924"
          fx="289.42923"
          fy="209.66924"
          r="92.5"
        />
        <radialGradient
          //@ts-ignore
          xlink:href="#linearGradient3263"
          id="radialGradient3349"
          gradientUnits="userSpaceOnUse"
          gradientTransform="matrix(0,2.8728591,-1,0,392.84375,-372.50696)"
          cx="199.28125"
          cy="192.84375"
          fx="199.28125"
          fy="192.84375"
          r="53.6875"
        />
        <radialGradient
          id="radialGradient3352"
          gradientUnits="userSpaceOnUse"
          gradientTransform="matrix(0,2.7782148,-0.82387859,0,358.87983,-353.64609)"
          cx="199.28125"
          cy="192.84375"
          fx="199.28125"
          fy="192.84375"
          r="68.84375"
        />
      </defs>
      <use
        x="0"
        y="0"
        //@ts-ignore
        xlink:href="#path3191"
        id="use31"
        transform="translate(2.71429,4)"
        width="100%"
        height="100%"
        style="fill-opacity:0.5;filter:url(#filter3315)"
      />
      <path
        id="path3191"
        d="M 200,15 C 97.88,15 15,97.88 15,200 15,302.12 97.88,385 200,385 302.12,385 385,302.12 385,200 385,97.88 302.12,15 200,15 Z m 0,179.8125 c 2.86645,0 5.18749,2.32105 5.1875,5.1875 0,2.86645 -2.32105,5.18749 -5.1875,5.1875 -2.86645,0 -5.18749,-2.32105 -5.1875,-5.1875 0,-2.86645 2.32105,-5.18749 5.1875,-5.1875 z"
      />
      <path
        id="path3271"
        d="m 200,132.65625 c -37.15925,0 -67.34375,30.1845 -67.34375,67.34375 0,37.15925 30.1845,67.34375 67.34375,67.34375 37.15925,0 67.34375,-30.1845 67.34375,-67.34375 0,-37.15925 -30.1845,-67.34375 -67.34375,-67.34375 z m 0,62.90625 c 2.45408,0 4.4375,1.98342 4.4375,4.4375 0,2.45408 -1.98342,4.4375 -4.4375,4.4375 -2.45408,0 -4.4375,-1.98342 -4.4375,-4.4375 0,-2.45408 1.98342,-4.4375 4.4375,-4.4375 z"
        style="fill:#333333"
      />
      <path
        id="path3176"
        d="M 200,16.78125 C 301.15637,16.78125 383.21875,98.843633 383.21875,200 383.21877,301.15637 301.15637,383.21875 200,383.21875 98.843633,383.21875 16.78125,301.15637 16.78125,200 16.78125,98.843633 98.843633,16.78125 200,16.78125 Z"
        style="fill:none;stroke:#000000;stroke-width:3.562"
      />
      <use
        height="100%"
        width="100%"
        transform="rotate(180,200,200)"
        id="use3151"
        //@ts-ignore
        xlink:href="#path3193"
        y="0"
        x="0"
      />
      <path
        id="path3193"
        d="m 106.4375,200 c 0,-51.65143 41.91107,-93.59375 93.5625,-93.59375 V 18.5625 C 99.85143,18.5625 18.56249,99.85142 18.5625,200 Z"
        style="fill:url(#radialGradient3328)"
      />
      <path
        id="path3215"
        d="M 200,132.6875 C 237.16701,132.6875 267.3125,162.83298 267.3125,200 267.3125,237.16701 237.16701,267.31445 200,267.31445 162.83299,267.31445 132.68555,237.16701 132.68555,200 132.68555,162.83298 162.83299,132.6875 200,132.6875 Z"
        style="fill:none;stroke:#000000;stroke-width:3"
      />
      <path
        id="path3223"
        d="m 237.9628,162.03721 c -20.95262,-20.9526 -54.97299,-20.95261 -75.9256,0 -20.9526,20.95262 -20.9526,54.97299 0,75.92559 20.95262,20.9526 54.97299,20.95261 75.9256,0 20.9526,-20.95261 20.9526,-54.97298 0,-75.92559 z m -34.29468,34.29468 c 2.02688,2.02689 2.02688,5.30934 0,7.33623 -2.02689,2.02689 -5.30934,2.02688 -7.33624,0 -2.02688,-2.02688 -2.02688,-5.30934 0,-7.33623 2.02689,-2.02688 5.30934,-2.02688 7.33624,0 z"
        style="fill:url(#radialGradient3349)"
      />
      <path
        id="path3230"
        d="m 243.95111,156.0489 c -7.06177,-7.06176 -15.40617,-12.05335 -24.2847,-15.00392 l -1.23744,3.71231 c 8.31532,2.76444 16.12455,7.46249 22.7379,14.07584 8.05626,8.05626 13.25594,17.88433 15.60055,28.24008 l 3.82279,-0.86178 c -2.50205,-11.05558 -8.03761,-21.56103 -16.6391,-30.16253 z"
        style="opacity:0.7425743;fill:#ffffff"
      />
      <path
        id="path3232"
        d="m 142.61386,190.25519 -3.88908,-0.66291 c -3.23715,19.17313 2.53847,39.5732 17.32411,54.35884 14.74595,14.74593 35.07597,20.5264 54.20416,17.34621 l -0.64082,-3.86699 c -17.92511,2.98388 -36.96183,-2.44619 -50.7791,-16.26346 -13.85228,-13.85228 -19.25338,-32.94694 -16.21927,-50.91169 z"
        style="opacity:0.7425743;fill:#ffffff"
      />
    </svg>
  );
}

export type PlayerProps = {};
