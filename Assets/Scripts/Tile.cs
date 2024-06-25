using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Tile : MonoBehaviour
{
    public bool isHilighted = false;
    public bool isOccupied = false;
    public string position = "";
    public int x;
    public int y;
    public Piece piece;
    public Color baseColor;

    public override string ToString()
    {
        return $"Tile {position} at ({x}, {y}), isOccupied: {isOccupied}";
    }

    // Start is called before the first frame update
    void Start()
    {

    }

    public void changeColor(Color color)
    {
        this.GetComponent<Renderer>().material.color = color;
    }
}

