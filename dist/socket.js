function start_socket() {
  const url = "ws://" + location.host + "/ws";
  const ws = new WebSocket(url);
  ws.onmessage = (ev) => {
    const msg = ev.data;
    const parsed = JSON.parse(msg);
    console.log(parsed);
  };
}

async function player_list() {
  const playlist = await fetch("/players");
  console.log(playlist);
}

function start_unity() {
    const template = document.querySelector("#unity-container-wrapper");
    const clone = template.content.cloneNode(true);
    document.body.appendChild(clone);
}
