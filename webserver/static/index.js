fetch('players').then(response => response.json()).then(json => loadPlayers(json))

let subBtn = document.getElementById("username-submit")
let userText = document.getElementById("username-input")

subBtn.onclick = function () {
    subBtn.disabled = true;
    fetch(`updatePlayer/${userText.value}`).then(response => response.text()).then(text => {
        alert(text)
        subBtn.disabled = false;
    })
    userText.value = ""
}

function loadPlayers(players) {
    let lb = document.getElementById("leaderboard")
    for (let i = 0; i < players.length; i++) {
        let player = players[i]
        let entry = document.createElement("div")
        entry.className = "leaderboard-entry"

        let pos = document.createElement("span")
        pos.className = "leaderboard-position"
        pos.innerHTML = ` ${i + 1})`

        let score = document.createElement("span")
        score.className = "leaderboard-totalscore"
        if (player.pb.replay) {
            let replay = document.createElement("a")
            replay.href = player.pb.replay
            replay.target = "_blank"
            replay.textContent = player.pb.blocks
            replay.style.color = "#ffff66"
            score.appendChild(replay)
        } else {
            score.textContent = player.pb.blocks
        }

        let flag = document.createElement("span")
        flag.className = "leaderboard-userphoto"
        if (player.country) {
            let img = document.createElement("img")
            img.src = `flags/${player.country["country-code"]}.png`
            img.crossOrigin = "anonymous"
            flag.appendChild(img)
        }

        let name = document.createElement("span")
        name.className = "leaderboard-name"
        let nameLink = document.createElement("a")
        nameLink.href = `https://jstris.jezevec10.com/u/${player.name}`
        nameLink.target = "_blank"
        nameLink.textContent = player.name
        name.appendChild(nameLink)

        let date = document.createElement("span")
        date.className = "leaderboard-date"
        let dateParts = player.pb.date.split(/[- :]/)
        date.textContent = Math.ceil((Date.now() - new Date(dateParts[0], dateParts[1]-1, dateParts[2], dateParts[3], dateParts[4], dateParts[5])) / (1e3 * 3600 * 24)) + " Days ago"

        entry.appendChild(pos)
        entry.appendChild(score)
        entry.appendChild(flag)
        entry.appendChild(name)
        entry.appendChild(date)

        lb.appendChild(entry)

    }
}

/*
<div class="leaderboard-entry">
<span class="leaderboard-position"> 1)</span>
<span class="leaderboard-totalscore">180</span>
<span class="leaderboard-userphoto"><img src="https://countryflagsapi.com/png/us" crossorigin="anonymous" height="20" /></span>
<a href="https://github.com/betaveros" target="_blank">betaveros</a>
</div>



        <div class="leaderboard-entry"><span class="leaderboard-position"> 1)</span> <span
                class="leaderboard-totalscore">180</span> <a href="https://github.com/betaveros" target="_blank"><span
                    class="leaderboard-userphoto"><img src="https://countryflagsapi.com/png/us" crossorigin="anonymous
" height="20" /></span>betaveros</a></div>
*/