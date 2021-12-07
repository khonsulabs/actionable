var searchIndex = JSON.parse('{\
"actionable":{"doc":"Actionable provides the basic functionality needed to …","t":[12,8,24,3,4,24,13,13,8,24,4,13,13,3,3,3,16,3,13,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,10,11,11,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12],"n":["0","Action","Action","ActionName","ActionNameList","Actionable","All","Any","Dispatcher","Dispatcher","Identifier","Integer","List","PermissionDenied","Permissions","ResourceName","Result","Statement","String","action","actions","allow_all","allow_all","allowed_to","and","any","as_ref","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","check","clone","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","clone_into","default","default","default","deserialize","deserialize","deserialize","deserialize","deserialize","deserialize","deserialize","dispatch","eq","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","hash","into","into","into","into","into","into","into","into_iter","merged","name","named","ne","resource","resources","serialize","serialize","serialize","serialize","serialize","serialize","serialize","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_string","to_string","to_string","to_string","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id","0","0","0"],"q":["actionable","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","actionable::ActionNameList","actionable::Identifier",""],"d":["","An action that can be allowed or disallowed.","Derives the <code>actionable::Action</code> trait.","A unique name of an action.","A list of <code>ActionName</code>s.","Derives a set of traits that can be used to implement a …","All actions.","When checking for allowed permissions, allow any match …","Dispatches <code>T</code> to an appropriate handler. This trait is …","Derives the <code>Dispatcher</code> trait.","A single element of a <code>ResourceName</code>","An integer identifier.","A specific list of names.","An <code>action</code> was denied.","A collection of allowed permissions. This is constructed …","A unique name/identifier of a resource.","The type of the result.","A statement of permissions. A statement describes whether …","A string identifier.","The <code>action</code> attempted upon <code>resource</code>.","The list of actions this statement applies to.","Returns a <code>Permisions</code> instance constructed with …","Returns a statement that allows <code>ActionNameList::All</code> against","Evaluate whether the <code>action</code> is allowed to be taken upon …","Adds another name segment.","Creates a <code>ResourceName</code> that matches any identifier.","","","","","","","","","","","","","","","","Evaluate whether the <code>action</code> is allowed to be taken upon …","","","","","","","","","","","","","","","","","","","","","Dispatches <code>request</code> to the appropriate handler while also …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns a new instance that merges all allowed actions from","The full name of this action.","Creates a <code>ResourceName</code> with <code>name</code>.","","The resource that <code>action</code> was attempted upon.","The list of resources this statement applies to.","","","","","","","","","","Convert this identifier to an un-borrowed identifier.","","Convert a borrowed name to an un-borrwed name.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[1,0,0,0,0,0,2,3,0,0,0,3,2,0,0,0,4,0,3,5,6,7,6,7,8,8,8,1,7,6,3,2,8,5,1,7,6,3,2,8,5,7,1,7,3,8,5,1,7,3,8,5,1,7,8,1,7,6,3,2,8,5,4,3,1,1,7,6,3,3,2,8,8,5,5,1,7,7,6,3,3,3,3,3,2,2,2,2,2,8,8,5,3,1,7,6,3,2,8,5,8,7,9,8,3,5,6,1,7,6,3,2,8,5,1,7,3,3,8,8,5,1,3,8,5,1,7,6,3,2,8,5,1,7,6,3,2,8,5,1,7,6,3,2,8,5,10,11,12],"f":[null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[]],[[]],[[["asref",8]],["bool",15]],[[["identifier",4],["into",8,["identifier"]]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["asref",8]],[["result",4,["permissiondenied"]],["permissiondenied",3]]],[[],["actionname",3]],[[],["permissions",3]],[[],["identifier",4]],[[],["resourcename",3]],[[],["permissiondenied",3]],[[]],[[]],[[]],[[]],[[]],[[],["actionname",3]],[[],["permissions",3]],[[],["resourcename",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["permissions",3]],[["pin",3,["box"]],["box",3,["future"]]]],[[["identifier",4]],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[["vec",3,["statement"]],["statement",3]]],[[]],[[["str",15]]],[[]],[[["string",3]]],[[["string",3]]],[[["u64",15]]],[[]],[[["vec",3,["actionname"]],["actionname",3]]],[[["actionname",3]]],[[["vec",3]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["actionname",3]],[[["identifier",4],["into",8,["identifier"]]]],[[["identifier",4]],["bool",15]],null,null,[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[]],[[]],[[],["identifier",4]],[[]],[[],["resourcename",3]],[[]],[[]],[[],["string",3]],[[],["string",3]],[[],["string",3]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null,null,null],"p":[[3,"ActionName"],[4,"ActionNameList"],[4,"Identifier"],[8,"Dispatcher"],[3,"PermissionDenied"],[3,"Statement"],[3,"Permissions"],[3,"ResourceName"],[8,"Action"],[13,"List"],[13,"Integer"],[13,"String"]]},\
"actionable_macros":{"doc":"Macros for the <code>actionable</code> API framework.","t":[24,24,24],"n":["Action","Actionable","Dispatcher"],"q":["actionable_macros","",""],"d":["Derives the <code>actionable::Action</code> trait.","Derives a set of traits that can be used to implement a …","Derives the <code>Dispatcher</code> trait."],"i":[0,0,0],"f":[null,null,null],"p":[]},\
"xtask":{"doc":"","t":[4,3,13,11,11,11,11,11,11,11,11,11,11,11,11,5,11,11,11,11,11,11,12],"n":["Commands","CoverageConfig","GenerateCodeCoverageReport","borrow","borrow","borrow_mut","borrow_mut","clap","fmt","from","from","from_clap","ignore_paths","into","into","main","try_from","try_from","try_into","try_into","type_id","type_id","install_dependencies"],"q":["xtask","","","","","","","","","","","","","","","","","","","","","","xtask::Commands"],"d":["","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,1,2,1,2,1,1,1,2,1,1,2,2,1,0,2,1,2,1,2,1,3],"f":[null,null,null,[[]],[[]],[[]],[[]],[[],["app",3]],[[["formatter",3]],["result",6]],[[]],[[]],[[["argmatches",3]]],[[],[["string",3],["vec",3,["string"]]]],[[]],[[]],[[],["result",6]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],null],"p":[[4,"Commands"],[3,"CoverageConfig"],[13,"GenerateCodeCoverageReport"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};