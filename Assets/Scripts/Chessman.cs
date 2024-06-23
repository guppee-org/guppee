using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Chessman : MonoBehaviour
{

    //Components
    private Animator animator;
    private CharacterController controller;
    private Vector3 targetPosition;

    //Properties
    public float jumpHeight = 10; // The max height the piece will reach while moving spaces.
    public int jumpSteps = 60; // 
    public float rotationSpeed = 1;
    public float maxAngle = 1f;
    private Quaternion startRotation;

    //TESTING VARIABLES
    public Transform target;



    // Start is called before the first frame update
    void Start()
    {
        animator = GetComponent<Animator>();
        controller = GetComponent<CharacterController>();
        startRotation = transform.rotation;
    }

    void Update()
    {
        if (Input.GetKeyDown(KeyCode.Space))
        {
            JumpTo(target.position);
        }
    }

    public void JumpTo(Vector3 _targetPosition)
    {
        this.targetPosition = _targetPosition;
        StopAllCoroutines();
        StartCoroutine(JumpSetupCoroutine(targetPosition));
    }

    IEnumerator JumpSetupCoroutine(Vector3 target)
    {
        yield return StartCoroutine(RotateTowardsTarget(target));

        //First part of jump animation begins. 
        //Once the first part of animation is complete it calls the StartJump function to begin the JumpCoroutine which moves the piece.
        animator.SetTrigger("Jump");
    }

    public void StartJump()
    {
        StartCoroutine(JumpCoroutine(targetPosition));
    }


    IEnumerator JumpCoroutine(Vector3 target)
    {
        yield return StartCoroutine(FollowCurve(target));
        yield return StartCoroutine(ResetRotation());
    }

    IEnumerator FollowCurve(Vector3 target)
    {
        QuadraticCurve movementCurve = new QuadraticCurve(transform.position, target, jumpHeight);

        for (int i = 0; i <= jumpSteps; i++)
        {
            Vector3 moveVector = movementCurve.Evaluate((float)i / jumpSteps) - transform.position;
            controller.Move(moveVector);
            yield return null;
        }

        animator.SetTrigger("Land");
    }

    IEnumerator RotateTowardsTarget(Vector3 target)
    {
        //find the vector pointing from our position to the target
        Vector3 targetDir = (target - transform.position).normalized;

        //create the rotation we need to be in to look at the target
        Quaternion lookRotation = Quaternion.LookRotation(targetDir);

        while (Vector3.Angle(transform.forward, targetDir.normalized) > maxAngle)
        {
            //rotate us over time according to speed until we are in the required rotation
            transform.rotation = Quaternion.Slerp(transform.rotation, lookRotation, Time.deltaTime * rotationSpeed);
            yield return null;
        }
    }

    IEnumerator ResetRotation()
    {
        while (Quaternion.Angle(transform.rotation, startRotation) > maxAngle)
        {
            transform.rotation = Quaternion.Slerp(transform.rotation, startRotation, Time.deltaTime * rotationSpeed);
            yield return null;
        }
    }
}
