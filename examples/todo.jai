OpenGl :: #foreign_system_library "OpenGl";
SomeForeignFunction :: (x: float) -> float #foreign OpenGl;

#scope_file
// things only scoped to the file here

#scope_export 
// things scoped globally here. this is the default

#load "filename.jai" rename_type #unshared

rename_type :: (x:string) -> string {
	//this allows renaming of types
}

#load "filename" x => concatenate(x, "t1")  #unshared;
#load "filename" x => concatenate(x, "t2")  #unshared;

/*
because of #unshared these are seperate instances of the imported filename
without unshared they point to the same thing and data
*/

#if OS_WIN {
//declare one thing
} else {
//declare another thing
}

/*
static if = regular if requires if boday and else body to be compiled, but static doesnt, only the compile time
branch needs to compile, this is only true syntactally not semantically - so that identifiers dont have to exist instances
none evaluating branch but it has to parse
Can also be used in the middle of structs to conditionally add fields
Same with functions
*/

// parse->sizing->typing->message-send->msg-recieve->ir->backend

//sizing queue:
// all elaments on struct go on sizing q and size independaently and when thety are done the struct is done recursively
// typig q - inference etc, building type info
