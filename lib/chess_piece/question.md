Hi !

I'm currently learning rust and started as a project the creation of a chess game.

I'm currently trying to invoke the following method of an object that has the trait ChessPiece which is a generic for Pawn, Bischop, Tower , etc ...

```rust
    fn check_moves<T : ChessPiece>(& mut self, board : &Board<T>) -> Vec<(i32,i32)>
    {
        self._allowed_moves.clear();

        // check forward tile , if not occupied move forward      
        let forward_move = (self._position.0, self._position.1+1);
        if let board.is_occupied(&forward_move) = None
        {
            self._allowed_moves.push(forward_move);
        }
        
        //check diagonals , if occupied by a piece of another team , proceed to eat
        let eat_moves = self.generate_moves();
        let i = 0 ;
        while let board.is_occupied(eat_moves.get(i)) = team
        {
            if self.team() != team
            {
                self._allowed_moves.push(eat_move.get(i));
            }
        }
    }
```

here is the signature of the method `board.is_occupied` :
```rust
    pub fn is_occupied( &self , position : &(i32,i32)) -> Option<Team>
```
if the Tile at given position is occupied it will return an Option of the team of occuping `ChessPiece`

However the `if let` and `while let` statements generate the same error : 
```console
error: expected one of `=`, `@`, or `|`, found `.`
  --> src/game/chess_piece/pawn.rs:68:21
   |
68 |         if let board.is_occupied(&forward_move) = None
   |                     ^ expected one of `=`, `@`, or `|`

error: expected one of `=`, `@`, or `|`, found `.`
```


However if refactor the problematic lines like below, the problem disappears.
```rust
        // check forward piece
        let some_piece : Option<Team> = board.is_occupied(&forward_move);
        if let some_piece = None
        {
            self._allowed_moves.push(forward_move);
        }
```

I DON'T UNDERSTAND WHY.

The types are the same, so why does `if let` doesn't accept invoking a method ?

Thanks in advance for your help  and have a nice day ! 