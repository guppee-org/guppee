using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class QuadraticCurve 
{
    public Vector3 A;
    public Vector3 B;
    public Vector3 C;

    public QuadraticCurve (Vector3 currentPos, Vector3 targetPos, float height){
        this.A = currentPos;
        this.B = targetPos;
        
        this.C = currentPos + (targetPos - currentPos) / 2;
        this.C += Vector3.up * height;
    }

    public Vector3 Evaluate(float t){
        Vector3 ac = Vector3.Lerp(A, C, t);
        Vector3 cb = Vector3.Lerp(C, B, t);
        return Vector3.Lerp(ac, cb, t);
    }
