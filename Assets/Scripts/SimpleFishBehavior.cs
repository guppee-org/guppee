using System.Collections;
using System.Collections.Generic;
using System.Runtime.CompilerServices;
using UnityEngine;

public class SimpleFishBehavior : MonoBehaviour
{
    public Transform target;
    private CharacterController controller;
    private Animator animator;

    [SerializeField]
    private float speed = 1;
    [SerializeField]
    private float rotationSpeed = 1;

    private Vector3 targetDir;

    private float maxAngle = 2.5f; //The largest angle that the fish will stop turning towards the target.

    // Start is called before the first frame update
    void Start()
    {
        controller = gameObject.GetComponent<CharacterController>();
        animator = gameObject.GetComponent<Animator>();
    }

    // Update is called once per frame
    void Update()
    {
        if (target){
            MoveToTarget();
        } else {

        }
    }

    private void MoveToTarget(){
        targetDir = (target.position - transform.position).normalized;

        if (Vector3.Angle(transform.forward, targetDir.normalized) > maxAngle){
            RotateTowardsTarget();
        } else {
            animator.ResetTrigger("TurnRight");
            animator.ResetTrigger("TurnLeft");
        }

        controller.Move(targetDir * speed * Time.deltaTime);
        
    }

    private void RotateTowardsTarget(){
        //find the vector pointing from our position to the target
        targetDir = (target.position - transform.position).normalized;

        //Calculate if we are turning to the left or right
        float angle = Vector3.SignedAngle(targetDir, transform.forward, transform.up);

        if (angle < -5.0F) {
            animator.SetTrigger("TurnRight");
        }
        else if (angle > 5.0F) {
            animator.SetTrigger("TurnLeft");
        }

        //create the rotation we need to be in to look at the target
        Quaternion lookRotation = Quaternion.LookRotation(targetDir);        

        //rotate us over time according to speed until we are in the required rotation
        transform.rotation = Quaternion.Slerp(transform.rotation, lookRotation, Time.deltaTime * rotationSpeed);
    }
}
