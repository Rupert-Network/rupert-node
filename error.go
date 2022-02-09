package main

import "errors"

var (
	ErrExpectedMoreParams = errors.New(ExpectedMoreParams)
	ErrOof                = errors.New(Oof)
)

const (
	ExpectedMoreParams = "expected more params"
	Oof                = "oof"
)
