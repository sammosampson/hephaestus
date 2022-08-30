/* OPERTOR PRECEDENCE:

* has priority over +
5 * 6 + 7

  *
  /\
 5  +
    /\
   6  7
   
   is wrong
   
   so rewrite tree to:
   
  +
  /\
 7  *
    /\
   5  6
*/

//ERROR HANDLING: 
//Bail on first error per file when lexxing, report all syntax errors though for undeclared identifier (?)

//FFI:
OpenGl :: #foreign_system_library "OpenGl";
SomeForeignFunction :: (x: float) -> float #foreign OpenGl;

//SCOPING:
#scope_file
// things only scoped to the file here
#scope_export 
// things scoped globally here. this is the default

//SRC FILE LOADING:
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

/*
STATIC IF:
static if = regular if requires if boday and else body to be compiled, but static doesnt, only the compile time
branch needs to compile, this is only true syntactally not semantically - so that identifiers dont have to exist instances
none evaluating branch but it has to parse
Can also be used in the middle of structs to conditionally add fields
Same with functions
*/
#if OS_WIN {
//declare one thing
} else {
//declare another thing
}

/*
COMPILER DESIGN:

vid: https://www.youtube.com/watch?v=MnctEW1oL-E&t=432s
(COMPILER DESIGN WITH CASEY)

Parse->Typing->Sizing->Message Send->Message Wait->IR->Run->Machine Code Gen->Exe

Typing: Type checking, Knowing types of things to do the work
Sizing: Only does stuff for structs to work out size
Message Send: Tells other people (eg metaprogram) about types of things
Message Wait: Wait for people to see it and ack
IR : Bytecode stage
Run: For running at compile time, you can run things here that are dependent down 
complete in lower stages, even though other things may not be complete. By complete
we mean they must have IR rep already.
This has to have cycle breaking in it too, because of infinite loops that can occur

the pipeline above is passed compilation units, ie top level statements such as declatrations, structs, methods
Parse splits these
Individual compilation units will hold at points in pipe if they are dependant on something, else they will pass through
ie will stop at typing if dep on type that is not yet passed a certain points 
Enums for example are comp units but so are their members, as they can depend on each other
But each line of a method is not an individual comp unit, as are only dependant top down

processing of compilation units is a bit like a game loop doing systems:

pipe - parse(file) { per parse(file) command in q => lex, make comp-units and put in table (comp-unit id, comp-unit), message "parse_complete" (comp unit id)  }
pipe - typing { per perform_typing(comp-unit-id) command in q => process each compilation unit and add complete types to typing table (comp-unit id, type_info)  and send  "typing_complete" (comp unit id) for them, and wait on any incomplete items in typing table (comp-unit id) }
pipe - sizing { per perform_sizing(comp-unit-id) command in q => process each compilation unit and add complete sizings to sizing table (comp-unit id, size  and send  "sizing_complete" (comp unit id) for them, and wait on any incomplete items in typing table (comp-unit id) }
pipe - ir-building { per perform_ir-building(comp-unit-id) }

work queues (parse, typing, sizing, ir-building, runnning) - they contain messages
loop {
	process work queues
		iterate through messages sending each to pipe or handler 
			
}

wait semaphore handler will halt executiion in a pipe until released by message
*/

//cargo run C:\work\other\rust-hephaestus\examples\calc.jai