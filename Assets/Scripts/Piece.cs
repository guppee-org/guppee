using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Piece : MonoBehaviour
{
    private string type = "";
    public Tile currentTile = null;
    private int pwnDirection = 1;
    private string color = "white";
    public int cx, cy;
    public Tile[,] board;

    // Start is called before the first frame update
    void Start()
    {


    }

    // Update is called once per frame
    void Update()
    {

    }

    void OnMouseEnter()
    {
        MoveSetPawn(board);
    }

    public void MoveSetPawn(Tile[,] board)
    {
        int cx = currentTile.x;
        int cy = currentTile.y;
        List<Tile> validEmptyTiles = new List<Tile>();
        List<Tile> validAttackTiles = new List<Tile>();


        if (color == "black")
        {
            if (board[cx, cy - 1].isOccupied == false)
            {
                validEmptyTiles.Add(board[cx, cy - 1]);
                if (board[cx, cy - 2].isOccupied == false)
                {
                    validEmptyTiles.Add(board[cx, cy - 2]);
                }
            }
            if (cx > 0 && board[cx - 1, cy - 1].isOccupied == true && board[cx - 1, cy - 1].piece.color != color)
            {
                validAttackTiles.Add(board[cx - 1, cy - 1]);
            }
            if (cx < 7 && board[cx + 1, cy - 1].isOccupied == true && board[cx + 1, cy - 1].piece.color != color)
            {
                validAttackTiles.Add(board[cx + 1, cy - 1]);
            }
        }
        else
        {
            if (board[cx, cy + 1].isOccupied == false)
            {
                validEmptyTiles.Add(board[cx, cy + 1]);
                if (board[cx, cy + 2].isOccupied == false)
                {
                    validEmptyTiles.Add(board[cx, cy + 2]);
                }
            }
            if (cx > 0 && board[cx + 1, cy + 1].isOccupied == true && board[cx + 1, cy + 1].piece.color != color)
            {
                validAttackTiles.Add(board[cx - 1, cy - 1]);
            }
            if (cx < 7 && board[cx + 1, cy + 1].isOccupied == true && board[cx + 1, cy + 1].piece.color != color)
            {
                validAttackTiles.Add(board[cx + 1, cy + 1]);
            }
        }



        validEmptyTiles.ForEach(delegate (Tile tile)
        {
            tile.changeColor(Color.green);
            Debug.Log(tile);
        });

        validAttackTiles.ForEach(delegate (Tile tile)
        {
            tile.changeColor(null);
        });
    }
}
