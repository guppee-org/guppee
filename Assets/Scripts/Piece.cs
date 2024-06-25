using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Piece : MonoBehaviour
{
    protected string type = "";
    public Tile currentTile = null;
    protected int pwnDirection = 1;
    protected string color = "white";
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
        Debug.Log("mouse enter");
        GetMoveSet(board);
    }

    void OnMouseExit()
    {
        Board board = GameObject.Find("BoardMain").GetComponent<Board>();
        if (board != null)
        {
            board.ResetTileColors();
        }
    }

    protected void GetMoveSet(Tile[,] board)
    {
        GetValidMoves(board);
    }

    protected virtual List<(int, int)> GetMoveOffsets()
    {
        return new List<(int, int)>();
    }

    protected virtual List<(int, int)> GetAttackOffsets()
    {
        return new List<(int, int)>();
    }


    protected void GetValidMoves(Tile[,] board)
    {
        List<(int, int)> moveOffsets = GetMoveOffsets();
        List<(int, int)> attackOffsets = GetAttackOffsets();

        Debug.Log("moves: " + string.Join(", ", moveOffsets));
        Debug.Log("attacks: " + string.Join(", ", attackOffsets));

        int cx = currentTile.x;
        int cy = currentTile.y;

        foreach (var offset in moveOffsets)
        {
            int newX = cx + offset.Item1;
            int newY = cy + offset.Item2;

            if (newX >= 0 && newX < 8 && newY >= 0 && newY < 8 && board[newX, newY].isOccupied == false)
            {
                board[newX, newY].changeColor(Color.green);
            }
        }

        foreach (var offset in attackOffsets)
        {
            int newX = cx + offset.Item1;
            int newY = cy + offset.Item2;

            if (newX >= 0 && newX < 8 && newY >= 0 && newY < 8 && board[newX, newY].isOccupied && board[newX, newY].piece.color != color)
            {
                board[newX, newY].changeColor(Color.red);
            }
        }
    }

}

