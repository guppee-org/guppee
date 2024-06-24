function start_socket() {
  const url = "ws://" + location.host + "/ws";
  const ws = new WebSocket(url);
  var whoami = null;

  ws.onmessage = (ev) => {
    if (whoami == null) {
      whoami = JSON.parse(ev.data);
      document.querySelector("#playerId").innerHTML = whoami.uuid;
      return;
    }
    player_list(ev.data);
  };
}

function player_list(data) {
  const players = JSON.parse(data);
  var tbody = document.querySelector("#playerListBody");
  tbody.innerHTML = ``;
  for (const player of players) {
    const content = `
      <tr>
        <td> ${player.uuid} </td>
        <td> 
          <button class="btn btn-primary btn-sm">Invite</button>
        </td>
      </tr>
    `;
    tbody.insertAdjacentHTML("beforeend", content);
  }
}

function start_unity() {
  const template = document.querySelector("#unity-container-wrapper");
  const clone = template.content.cloneNode(true);
  document.body.appendChild(clone);
}
