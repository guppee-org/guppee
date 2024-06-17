using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class rotater : MonoBehaviour
{
    public Rigidbody rb;
    // Start is called before the first frame update
    void Start()
    {
        rb = gameObject.GetComponent<Rigidbody>();
        rb.AddForce(0, 1000, 0);
    }

    void OnMouseDown()
    {
        rb.AddForce(0, 200, 0);
    }

    // Update is called once per frame
    void Update()
    {
    }
}
