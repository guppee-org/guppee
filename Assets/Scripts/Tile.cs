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

    // void OnMouseEnter()
    // {
    //     isHilighted = true;
    // }
    //
    // void OnMouseExit()
    // {
    //     isHilighted = false;
    // }
    //
    // Update is called once per frame
    void Update()
    {
        // if (isHilighted)
        // {
        //     this.GetComponent<Renderer>().material.color = Color.green;
        // }
        // else
        // {
        //     if (x % 2 == 0)
        //     {
        //         if (y % 2 == 0)
        //         {
        //             this.GetComponent<Renderer>().material.color = Color.white;
        //         }
        //         else
        //         {
        //             this.GetComponent<Renderer>().material.color = Color.black;
        //         }
        //     }
        //     else
        //     {
        //         if (y % 2 == 0)
        //         {
        //             this.GetComponent<Renderer>().material.color = Color.black;
        //         }
        //         else
        //         {
        //             this.GetComponent<Renderer>().material.color = Color.white;
        //         }
        //     }
        // }
        //
    }

    public void changeColor(Color color)
    {
        this.GetComponent<Renderer>().material.color = color;
    }
}

