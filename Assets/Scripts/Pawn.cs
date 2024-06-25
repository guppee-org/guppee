using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Pawn : Piece
{
    private bool isFirstMove = true;
    // Start is called before the first frame update
    void Start()
    {

    }

    // Update is called once per frame
    void Update()
    {

    }

    protected override List<(int, int)> GetMoveOffsets()
    {
        int direction = color == "black" ? -1 : 1;
        var offsets = new List<(int, int)> { (0, direction) };
        if (isFirstMove)
        {
            offsets.Add((0, 2 * direction));
        }

        return offsets;
    }

    protected override List<(int, int)> GetAttackOffsets()
    {
        int direction = color == "black" ? -1 : 1;
        return new List<(int, int)> { (-1, direction), (1, direction) };
    }
}
