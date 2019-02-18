import { Engine, PlayerCore } from './jsint_roguewasm';

class Player {
  constructor (game, x, y) {
    this.game = game
    this._core = new PlayerCore(x, y, '@', '#ff0', game.display)
    this._core.draw()
  }

  act () {
    this.game.rotengine.lock()
    window.addEventListener('keydown', this.handleEvent.bind(this))
  }

  handleEvent (e) {
    const keyMap = {
      38: 0,
      33: 1,
      39: 2,
      34: 3,
      40: 4,
      35: 5,
      37: 6,
      36: 7
    }

    const code = e.keyCode
    if (code === 13 || code === 32) {
      this.game.engine.open_box(this._core, this._core.x(), this._core.y())
      return
    }

    if (!keyMap[code]) {
      return
    }

    const dir = ROT.DIRS[8][keyMap[code]]
    const newX = this._core.x() + dir[0]
    const newY = this._core.y() + dir[1]

    if (!this.game.engine.free_cell(newX, newY)) {
      return
    }

    this.game.engine.move_player(this._core, newX, newY)
    window.removeEventListener('keydown', this)
    this.game.rotengine.unlock()
  }

  getX() {
    this._core.x()
  }

  getY() {
    this._core.y()
  }
}

class Checko {
  constructor (game, x, y) {
    this.game = game
    this._core = new PlayerCore(x, y, 'B', '#f00', this.game.display)
    this._core.draw()
  }

  act () {
    const x = this.game.player.getX()
    const y = this.game.player.getY()

    const passableCallback = () => this.game.engine.free_cell(x, y)
    const astar = new ROT.Path.AStar(x, y, passableCallback, { topology: 4 })

    const path = []
    const pathCallback = (x, y) => path.push([x, y])
    astar.compute(this._core.x(), this._core.y(), pathCallback)

    path.shift()
    if (path.length <= 1) {
      this.game.rotengine.lock()
      alert('Game over')
    } else {
      x = path[0][0]
      y = path[0][1]

      this.game.engine.move_player(this._core, x, y)
    }
  }
}

class Game {
  constructor() {
    this.display = new ROT.Display();
    document.getElementById("rogueCanvas").appendChild(this.display.getContainer())

    this.engine = new Engine(ROT.RNG, this.display)
    this.generateMap()

    var scheduler = new ROT.Scheduler.Simple()

    scheduler.add(this.player, true)
    scheduler.add(this.enemy, true)

    this.rotengine = new ROT.Engine(scheduler)
    this.rotengine.start()
  }

  _createBeing (what, freeCells) {
    const index = Math.floor(ROT.RNG.getUniform() * freeCells.length)
    const key = freeCells.splice(index, 1)[0];
    const parts = key.split(",");
    const x = parseInt(parts[0]);
    const y = parseInt(parts[1]);
    return new what(this, x, y);
  }

  generateMap () {
    const digger = new ROT.Map.Digger()
    let freeCells = []
    const digCallback = (x, y, value) => {
      if (!value) {
        freeCells.push([x, y]);
      }
      this.engine.on_dig(x, y, value)
    }

    digger.create(digCallback.bind(this))
    freeCells = this.engine.generate_boxes(freeCells)
    this.engine.draw_map()

    this.player = this._createBeing(Player, freeCells)
    this.enemy = this._createBeing(Checko, freeCells)
  }

  generatePlayer (freeCells) {
    var index = Math.floor(ROT.RNG.getUniform() * freeCells.length);
    var key = freeCells.splice(index, 1)[0];
    var parts = key.split(",");
    var x = parseInt(parts[0]);
    var y = parseInt(parts[1]);

    console.log("Generating player...");
    this.player = new Player(x, y);
  }
}

new Game()

export function stats_updated(stats) {
  document.getElementById("hitpoints").textContent = stats.hitpoints
  document.getElementById("max_hitpoints").textContent = stats.max_hitpoints
  document.getElementById("moves").textContent = stats.moves
}
