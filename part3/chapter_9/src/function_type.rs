trait FunctionSignature {
    fn function_signature<PARAM1, PARAM2, RESULT>(param: PARAM1) -> dyn Fn(PARAM2) -> RESULT;
}
