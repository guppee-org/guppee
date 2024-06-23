function start_socket() {
  const url = "ws://" + location.host + "/ws";
  const ws = new WebSocket(url);
  ws.onmessage = (ev) => {
    const msg = ev.data;
    const parsed = JSON.parse(msg);
    console.log(parsed);
  };
}

function player_list() {

}
