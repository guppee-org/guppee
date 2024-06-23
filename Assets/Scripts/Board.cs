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
                GameObject tile = GameObject.CreatePrimitive(PrimitiveType.Cube);
                tile.transform.position = new Vector3((i * cellSize) - offset, boardHeight, (j * cellSize) - offset);
                tile.transform.localScale = new Vector3(cellSize, 0.1f, cellSize);
                tile.transform.parent = this.transform;
                tile.AddComponent<Tile>();
                tile.GetComponent<Tile>().x = i;
                tile.GetComponent<Tile>().y = j;
                tile.GetComponent<Tile>().position = i.ToString() + j.ToString();
                board[i, j] = tile.GetComponent<Tile>();
                if (color)
                {
                    tile.GetComponent<Renderer>().material.color = Color.black;
                }
                else
                {
                    tile.GetComponent<Renderer>().material.color = Color.white;
                }
                color = !color;
            }
        }
        // board[0, 0].piece = new Piece(PieceType.ROOK, true);
        Piece pawn = Instantiate(pawnPrefab, board[0, 0].transform.position, transform.rotation).AddComponent(typeof(Piece)) as Piece;
        pawn.board = board;
        pawn.currentTile = board[0, 0];
        board[0, 0].piece = pawn.GetComponent<Piece>();
    }

    // Update is called once per frame
    void Update()
    {

    }
}
