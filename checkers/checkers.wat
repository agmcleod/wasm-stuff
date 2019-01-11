(module
  (import "events" "piecemoved"
    (func $notify_piecemoved (param $fromX i32) (param $fromY i32) (param $toX i32) (param $toY i32))
  )
  (import "events" "piececrowned"
    (func $notify_piececrowned (param $pieceX i32) (param $pieceY i32))
  )

  (import "events" "errormove"
    (func $notify_errormove (param $fromX i32) (param $fromY i32) (param $toX i32) (param $toY i32))
  )

  (memory $mem 1)
  (global $currentTurn (mut i32) (i32.const 0))
  (global $WHITE i32 (i32.const 2))
  (global $BLACK i32 (i32.const 1))
  (global $CROWN i32 (i32.const 4))
  (func $indexForPosition (param $x i32) (param $y i32) (result i32)
    (i32.add
      (i32.mul
        (i32.const 8)
        (get_local $y)
      )
      (get_local $x)
    )
  )
  ;; Offset = (x + y * 8) + 4
  (func $offsetForPosition (param $x i32) (param $y i32) (result i32)
    (i32.mul
      (call $indexForPosition (get_local $x) (get_local $y))
      (i32.const 4)
    )
  )

  ;; Determine if a piece was crowed
  (func $isCrowned (param $piece i32) (result i32)
    (i32.eq
      (i32.and (get_local $piece) (get_global $CROWN))
      (get_global $CROWN)
    )
  )

  ;; Determine if a piece is white
  (func $isWhite (param $piece i32) (result i32)
    (i32.eq
      (i32.and (get_local $piece) (get_global $WHITE))
      (get_global $WHITE)
    )
  )

  ;; Determine if a piece is black
  (func $isBlack (param $piece i32) (result i32)
    (i32.eq
      (i32.and (get_local $piece) (get_global $BLACK))
      (get_global $BLACK)
    )
  )

  ;; Adds a crown to a given piece (no mutation)
  (func $withCrown (param $piece i32) (result i32)
    (i32.or (get_local $piece) (get_global $CROWN))
  )

  ;; Removes a crown from a given piece (no mutation)
  (func $withoutCrown (param $piece i32) (result i32)
    (i32.and (get_local $piece) (i32.const 3))
  )

  ;; Sets a piece on a board
  (func $setPiece (param $x i32) (param $y i32) (param $piece i32)
    (i32.store
      (call $offsetForPosition
        (get_local $x)
        (get_local $y)
      )
      (get_local $piece)
    )
  )

  ;; Gets a piece from the board. Out of range causes a trap
  (func $getPiece (param $x i32) (param $y i32) (result i32)
    (if (result i32)
      (block (result i32)
        (i32.and
          (call $inRange
            (i32.const 0)
            (i32.const 7)
            (get_local $x)
          )
          (call $inRange
            (i32.const 0)
            (i32.const 7)
            (get_local $y)
          )
        )
      )
      (then
        (i32.load
          (call $offsetForPosition
            (get_local $x)
            (get_local $y)
          )
        )
      )
      (else
        (unreachable)
      )
    )
  )

  ;; Determine if a space is occupied without regard for who occupies it
  (func $isOccupied (param $x i32) (param $y i32) (result i32)
    (i32.gt_s
      (call $getPiece (get_local $x) (get_local $y))
      (i32.const 0)
    )
  )

  ;; Detect if values are within range
  (func $inRange (param $low i32) (param $high i32) (param $value i32) (result i32)
    (i32.and
      (i32.ge_s (get_local $value) (get_local $low))
      (i32.le_s (get_local $value) (get_local $high))
    )
  )

  ;; Get current turn owner
  (func $getTurnOwner (result i32)
    (get_global $currentTurn)
  )

  ;; At the end of a turn, switch turn owner to the other player
  (func $toggleTurnOwner
    (if (i32.eq (call $getTurnOwner) (i32.const 1))
      (then (call $setTurnOwner (i32.const 2)))
      (else (call $setTurnOwner (i32.const 1)))
    )
  )

  ;; Sets the turn owner
  (func $setTurnOwner (param $piece i32)
    (set_global $currentTurn (get_local $piece))
  )

  ;; Determine if it's a player's turn
  (func $isPlayersTurn (param $player i32) (result i32)
    (i32.gt_s
      (i32.and (get_local $player) (call $getTurnOwner))
      (i32.const 0)
    )
  )

  ;; Should this piece get crowned
  ;; We crown black pieces in row 0, with pieces in row 7
  (func $shouldCrown (param $pieceY i32) (param $piece i32) (result i32)
    (i32.or
      (i32.and
        (i32.eq
          (get_local $pieceY)
          (i32.const 0)
        )
        (call $isBlack (get_local $piece))
      )
      (i32.and
        (i32.eq
          (get_local $pieceY)
          (i32.const 7)
        )
        (call $isWhite (get_local $piece))
      )
    )
  )

  ;; Converts a piece into a crowned piece and invokes a host notifier
  (func $crownPiece (param $x i32) (param $y i32)
    (local $piece i32)
    (set_local $piece (call $getPiece (get_local $x) (get_local $y)))
    (call $setPiece (get_local $x) (get_local $y) (call $withCrown (get_local $piece)))
    (call $notify_piececrowned (get_local $x) (get_local $y))
  )

  (func $distance (param $x i32) (param $y i32) (result i32)
    (i32.sub
      (get_local $x)
      (get_local $y)
    )
  )

  ;; Determine if the move is valid
  (func $isValidMove (param $fromX i32) (param $fromY i32) (param $toX i32) (param $toY i32) (result i32)
    (local $player i32)
    (local $target i32)
    (set_local $player (call $getPiece (get_local $fromX) (get_local $fromY)))
    (set_local $target (call $getPiece (get_local $toX) (get_local $toY)))
    (if (result i32)
      (block (result i32)
        (i32.and
          (call $validJumpDistance (get_local $fromY) (get_local $toY))
          (i32.and
            (call $isPlayersTurn (get_local $player))
            (i32.eq
              (get_local $target)
              (i32.const 0) ;; target is unoccupied
            )
          )
        )
      )
      (then
        (i32.const 1)
      )
      (else
        (i32.const 0)
      )
    )
  )

  ;; Ensures travel is 1 or 2 squares
  (func $validJumpDistance (param $fromY i32) (param $toY i32) (result i32)
    (local $d i32)
    (set_local $d
      (if (result i32)
        (i32.gt_s (get_local $toY) (get_local $fromY))
        (then
          (call $distance (get_local $toY) (get_local $fromY))
        )
        (else
          (call $distance (get_local $fromY) (get_local $toY))
        )
      )
    )
    (i32.le_u
      (get_local $d)
      (i32.const 2)
    )
  )

  ;; Exported move function
  (func $move (param $fromX i32) (param $fromY i32) (param $toX i32) (param $toY i32) (result i32)
    (if (result i32)
      (block (result i32)
        (call $isValidMove (get_local $fromX) (get_local $fromY) (get_local $toX) (get_local $toY))
      )
      (then
        (call $doMove (get_local $fromX) (get_local $fromY) (get_local $toX) (get_local $toY))
      )
      (else
        (call $notify_errormove (get_local $fromX) (get_local $fromY) (get_local $toX) (get_local $toY))
        (i32.const 0)
      )
    )
  )

  (func $doMove (param $fromX i32) (param $fromY i32) (param $toX i32) (param $toY i32) (result i32)
    (local $curpiece i32)
    (set_local $curpiece (call $getPiece (get_local $fromX) (get_local $fromY)))
    (call $toggleTurnOwner)
    (call $setPiece (get_local $toX) (get_local $toY) (get_local $curpiece))
    (call $setPiece (get_local $fromX) (get_local $fromY) (i32.const 0))
    (if (call $shouldCrown (get_local $toX) (get_local $toY))
      (then (call $crownPiece (get_local $toX) (get_local $toY)))
    )
    (call $notify_piecemoved (get_local $fromX) (get_local $fromY) (get_local $toX) (get_local $toY))
    (i32.const 1)
  )

  (func $initBoard
    (call $setPiece (i32.const 1) (i32.const 0) (get_global $WHITE))
    (call $setPiece (i32.const 3) (i32.const 0) (get_global $WHITE))
    (call $setPiece (i32.const 5) (i32.const 0) (get_global $WHITE))
    (call $setPiece (i32.const 7) (i32.const 0) (get_global $WHITE))
    (call $setPiece (i32.const 0) (i32.const 1) (get_global $WHITE))
    (call $setPiece (i32.const 2) (i32.const 1) (get_global $WHITE))
    (call $setPiece (i32.const 3) (i32.const 1) (get_global $WHITE))
    (call $setPiece (i32.const 6) (i32.const 1) (get_global $WHITE))

    (call $setPiece (i32.const 1) (i32.const 7) (get_global $BLACK))
    (call $setPiece (i32.const 3) (i32.const 7) (get_global $BLACK))
    (call $setPiece (i32.const 5) (i32.const 7) (get_global $BLACK))
    (call $setPiece (i32.const 7) (i32.const 7) (get_global $BLACK))
    (call $setPiece (i32.const 0) (i32.const 6) (get_global $BLACK))
    (call $setPiece (i32.const 2) (i32.const 6) (get_global $BLACK))
    (call $setPiece (i32.const 3) (i32.const 6) (get_global $BLACK))
    (call $setPiece (i32.const 6) (i32.const 6) (get_global $BLACK))

    (call $setTurnOwner (i32.const 1))
  )

  (export "getPiece" (func $getPiece))
  (export "isOccupied" (func $isOccupied))
  (export "isCrowned" (func $isCrowned))
  (export "initBoard" (func $initBoard))
  (export "getTurnOwner" (func $getTurnOwner))
  (export "move" (func $move))
  (export "memory" (memory $mem))
)