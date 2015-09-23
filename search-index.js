var searchIndex = {};
searchIndex['io_providers'] = {"items":[[0,"","io_providers","Defines \"provider\" traits and implementations for different types of I/O operations.",null,null],[3,"LocalIoProvider","","\"Real\" implementer of `IoProvider`, using standard streams and the local environment.",null,null],[3,"VirtualIoProvider","","Virtual implementer of `IoProvider`, using in-memory data which can be inspected.",null,null],[0,"env","","Providers of environment data, such as the working directory and environment variables.",null,null],[3,"Local","io_providers::env","Provides access to the local environment (e.g. what the corresponding `std::env` functions\nwould access).",null,null],[3,"Virtual","","Provides access to a virtual environment, which can be configured independently from the\nlocal system.",null,null],[11,"new","","Creates a new local environment provider.",0,{"inputs":[{"name":"local"}],"output":{"name":"local"}}],[11,"args","","",0,{"inputs":[{"name":"local"}],"output":{"name":"vec"}}],[11,"current_dir","","",0,{"inputs":[{"name":"local"}],"output":{"name":"result"}}],[11,"set_current_dir","","",0,{"inputs":[{"name":"local"},{"name":"path"}],"output":{"name":"result"}}],[11,"new","","Creates a new virtual environment.",1,{"inputs":[{"name":"virtual"}],"output":{"name":"virtual"}}],[11,"set_args","","Sets the arguments.",1,{"inputs":[{"name":"virtual"},{"name":"vec"}],"output":null}],[11,"args","","",1,{"inputs":[{"name":"virtual"}],"output":{"name":"vec"}}],[11,"current_dir","","",1,{"inputs":[{"name":"virtual"}],"output":{"name":"result"}}],[11,"set_current_dir","","",1,{"inputs":[{"name":"virtual"},{"name":"path"}],"output":{"name":"result"}}],[8,"Provider","","Provides access to environment data, such as working directory and environment variables.",null,null],[10,"args","","Returns the arguments which this program was started with (normally passed via the command\nline).",2,{"inputs":[{"name":"provider"}],"output":{"name":"vec"}}],[10,"current_dir","","Returns the current working directory as a `PathBuf`.",2,{"inputs":[{"name":"provider"}],"output":{"name":"result"}}],[10,"set_current_dir","","Changes the current working directory to the specified path, returning whether the change\nwas completed successfully or not.",2,{"inputs":[{"name":"provider"},{"name":"path"}],"output":{"name":"result"}}],[0,"stream","io_providers","Providers of input/output/error streams (i.e. stdin, stdout and stderr).",null,null],[3,"Std","io_providers::stream","Provides access to the standard streams (stdin, stdout and stderr).",null,null],[3,"Virtual","","Provides virtual input/output/error streams: input can be provided using\n`Virtual::write_input()`, and output can be observed using `Virtual::read_output()` and\n`Virtual::read_error()`.",null,null],[11,"new","","Constructs a new standard stream provider.",3,{"inputs":[{"name":"std"}],"output":{"name":"std"}}],[11,"input","","",3,{"inputs":[{"name":"std"}],"output":{"name":"read"}}],[11,"output","","",3,{"inputs":[{"name":"std"}],"output":{"name":"write"}}],[11,"error","","",3,{"inputs":[{"name":"std"}],"output":{"name":"write"}}],[11,"new","","Creates a new, empty virtual stream provider.",4,{"inputs":[{"name":"virtual"}],"output":{"name":"virtual"}}],[11,"write_input","","Writes the provided buffer to the queue of buffers to be used when input is requested\nfrom this provider using `Provider::input()`.",4,null],[11,"read_output","","Gets the data which has been written to the output stream.",4,null],[11,"read_error","","Gets the data which has been written to error stream.",4,null],[11,"input","","",4,{"inputs":[{"name":"virtual"}],"output":{"name":"read"}}],[11,"output","","",4,{"inputs":[{"name":"virtual"}],"output":{"name":"write"}}],[11,"error","","",4,{"inputs":[{"name":"virtual"}],"output":{"name":"write"}}],[8,"Provider","","Provides access to input, output and error streams.",null,null],[10,"input","","Gets the input stream.",5,{"inputs":[{"name":"provider"}],"output":{"name":"read"}}],[10,"output","","Gets the output stream.",5,{"inputs":[{"name":"provider"}],"output":{"name":"write"}}],[10,"error","","Gets the error stream.",5,{"inputs":[{"name":"provider"}],"output":{"name":"write"}}],[8,"IoProvider","io_providers","Provides access to an environment provider and a stream provider.",null,null],[10,"env","","Gets the `env::Provider`.",6,{"inputs":[{"name":"ioprovider"}],"output":{"name":"provider"}}],[10,"stream","","Gets the `stream::Provider`.",6,{"inputs":[{"name":"ioprovider"}],"output":{"name":"provider"}}],[11,"new","","Creates a new `LocalIoProvider`.",7,{"inputs":[{"name":"localioprovider"}],"output":{"name":"localioprovider"}}],[11,"env","","",7,{"inputs":[{"name":"localioprovider"}],"output":{"name":"provider"}}],[11,"stream","","",7,{"inputs":[{"name":"localioprovider"}],"output":{"name":"provider"}}],[11,"new","","Creates a new `VirtualIoProvider`.",8,{"inputs":[{"name":"virtualioprovider"}],"output":{"name":"virtualioprovider"}}],[11,"virtual_env","","Gets the `env::Virtual` provider.",8,{"inputs":[{"name":"virtualioprovider"}],"output":{"name":"virtual"}}],[11,"virtual_stream","","Gets the `stream::Virtual` provider.",8,{"inputs":[{"name":"virtualioprovider"}],"output":{"name":"virtual"}}],[11,"env","","",8,{"inputs":[{"name":"virtualioprovider"}],"output":{"name":"provider"}}],[11,"stream","","",8,{"inputs":[{"name":"virtualioprovider"}],"output":{"name":"provider"}}]],"paths":[[3,"Local"],[3,"Virtual"],[8,"Provider"],[3,"Std"],[3,"Virtual"],[8,"Provider"],[8,"IoProvider"],[3,"LocalIoProvider"],[3,"VirtualIoProvider"]]};
initSearch(searchIndex);