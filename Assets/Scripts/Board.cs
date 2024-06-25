using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Board : MonoBehaviour
{
    public Tile[,] board = new Tile[8, 8];
    public Collider boardCollider;
    private Vector3 boardBounds;
    public float boardHeight = 0.6f;
    public GameObject pawnPrefab;

    // Start is called before the first frame update
    void Start()
    {
        boardBounds = boardCollider.bounds.size;
        float cellSize = boardBounds.x / 8;
        float offset = boardBounds.x / 2 - cellSize / 2;
        Debug.Log("cellSize: " + cellSize + " offset: " + offset);
        Debug.Log("boardBounds: " + boardBounds);
        bool color = false;

        for (int i = 0; i < 8; i++)
        {
            color = !color;
            for (int j = 0; j < 8; j++)
            {
                GameObject tileGO = GameObject.CreatePrimitive(PrimitiveType.Cube);
                tileGO.transform.position = new Vector3((i * cellSize) - offset, boardHeight, (j * cellSize) - offset);
                tileGO.transform.localScale = new Vector3(cellSize, 0.1f, cellSize);
                tileGO.transform.parent = this.transform;
                Tile tile = tileGO.AddComponent<Tile>();
                tile.x = i;
                tile.y = j;
                tile.position = i.ToString() + j.ToString();
                board[i, j] = tile.GetComponent<Tile>();
                if (color)
                {
                    tile.baseColor = Color.black;
                    tile.changeColor(Color.black);
                }
                else
                {
                    tile.baseColor = Color.white;
                    tile.changeColor(Color.white);
                }
                color = !color;
            }
        }
        // board[0, 0].piece = new Piece(PieceType.ROOK, true);
        Pawn pawn = Instantiate(pawnPrefab, board[0, 0].transform.position, transform.rotation).AddComponent(typeof(Pawn)) as Pawn;
        pawn.board = board;
        pawn.currentTile = board[0, 0];
        board[0, 0].piece = pawn.GetComponent<Pawn>();
    }

    // Update is called once per frame
    void Update()
    {

    }

    public void ResetTileColors()
    {
        foreach (var tile in board)
        {
            tile.changeColor(tile.baseColor);
        }
    }
}
