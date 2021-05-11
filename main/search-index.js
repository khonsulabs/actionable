var searchIndex = JSON.parse('{\
"actionable":{"doc":"Actionable provides the basic functionality needed to …","i":[[8,"Action","actionable","An action that can be allowed or disallowed.",null,null],[10,"name","","The full name of this action.",0,[[],["actionname",3]]],[24,"Action","","Derives the <code>actionable::Action</code> trait.",null,null],[3,"ActionName","","A unique name of an action.",null,null],[12,"0","","",1,null],[8,"Dispatcher","","Dispatches <code>T</code> to an appropriate handler. This trait is …",null,null],[16,"Result","","The type of the result.",2,null],[10,"dispatch","","Dispatches <code>request</code> to the appropriate handler while also …",2,[[["permissions",3]],[["pin",3],["box",3]]]],[24,"Dispatcher","","Derives the <code>Dispatcher</code> trait.",null,null],[3,"Permissions","","A collection of allowed permissions. This is constructed …",null,null],[4,"ActionNameList","","A list of [<code>ActionName</code>]s.",null,null],[13,"List","","A specific list of names.",3,null],[13,"All","","All actions.",3,null],[4,"Identifier","","A single element of a [<code>ResourceName</code>]",null,null],[13,"Any","","When checking for allowed permissions, allow any match …",4,null],[13,"Integer","","An integer identifier.",4,null],[13,"String","","A string identifier.",4,null],[3,"ResourceName","","A unique name/identifier of a resource.",null,null],[3,"Statement","","A statement of permissions. A statement describes whether …",null,null],[12,"resources","","The list of resources this statement applies to.",5,null],[12,"actions","","The list of actions this statement applies to.",5,null],[12,"allowed","","Whether the <code>actions</code> should be allowed or disallowed.",5,null],[24,"Actionable","","Derives a set of traits that can be used to implement a …",null,null],[3,"PermissionDenied","","An <code>action</code> was denied.",null,null],[12,"resource","","The resource that <code>action</code> was attempted upon.",6,null],[12,"action","","The <code>action</code> attempted upon <code>resource</code>.",6,null],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"to_owned","","",1,[[]]],[11,"clone_into","","",1,[[]]],[11,"to_string","","",1,[[],["string",3]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"from","","",7,[[]]],[11,"into","","",7,[[]]],[11,"borrow","","",7,[[]]],[11,"borrow_mut","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"try_into","","",7,[[],["result",4]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"from","","",5,[[]]],[11,"into","","",5,[[]]],[11,"borrow","","",5,[[]]],[11,"borrow_mut","","",5,[[]]],[11,"try_from","","",5,[[],["result",4]]],[11,"try_into","","",5,[[],["result",4]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"from","","",4,[[]]],[11,"into","","",4,[[]]],[11,"to_owned","","",4,[[]]],[11,"clone_into","","",4,[[]]],[11,"to_string","","",4,[[],["string",3]]],[11,"borrow","","",4,[[]]],[11,"borrow_mut","","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"try_into","","",4,[[],["result",4]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"from","","",3,[[]]],[11,"into","","",3,[[]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"from","","",8,[[]]],[11,"into","","",8,[[]]],[11,"to_owned","","",8,[[]]],[11,"clone_into","","",8,[[]]],[11,"to_string","","",8,[[],["string",3]]],[11,"borrow","","",8,[[]]],[11,"borrow_mut","","",8,[[]]],[11,"try_from","","",8,[[],["result",4]]],[11,"try_into","","",8,[[],["result",4]]],[11,"type_id","","",8,[[],["typeid",3]]],[11,"from","","",6,[[]]],[11,"into","","",6,[[]]],[11,"to_owned","","",6,[[]]],[11,"clone_into","","",6,[[]]],[11,"to_string","","",6,[[],["string",3]]],[11,"borrow","","",6,[[]]],[11,"borrow_mut","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"try_into","","",6,[[],["result",4]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"as_ref","","",8,[[]]],[11,"from","","",7,[[["vec",3],["statement",3]]]],[11,"from","","",4,[[["u64",15]]]],[11,"from","","",4,[[["str",15]]]],[11,"from","","",4,[[["string",3]]]],[11,"from","","",4,[[["string",3]]]],[11,"from","","",3,[[]]],[11,"from","","",3,[[["actionname",3]]]],[11,"from","","",3,[[["vec",3],["actionname",3]]]],[11,"into_iter","","",8,[[]]],[11,"clone","","",1,[[],["actionname",3]]],[11,"clone","","",4,[[],["identifier",4]]],[11,"clone","","",8,[[],["resourcename",3]]],[11,"clone","","",6,[[],["permissiondenied",3]]],[11,"default","","",1,[[],["actionname",3]]],[11,"default","","",7,[[],["permissions",3]]],[11,"default","","",8,[[],["resourcename",3]]],[11,"eq","","",4,[[["identifier",4]],["bool",15]]],[11,"ne","","",4,[[["identifier",4]],["bool",15]]],[11,"fmt","","",1,[[["formatter",3]],["result",6]]],[11,"fmt","","",7,[[["formatter",3]],["result",6]]],[11,"fmt","","",5,[[["formatter",3]],["result",6]]],[11,"fmt","","",4,[[["formatter",3]],["result",6]]],[11,"fmt","","",3,[[["formatter",3]],["result",6]]],[11,"fmt","","",8,[[["formatter",3]],["result",6]]],[11,"fmt","","",6,[[["formatter",3]],["result",6]]],[11,"fmt","","",1,[[["formatter",3]],["result",6]]],[11,"fmt","","",4,[[["formatter",3]],["result",6]]],[11,"fmt","","",8,[[["formatter",3]],["result",6]]],[11,"fmt","","",6,[[["formatter",3]],["result",6]]],[11,"hash","","",4,[[]]],[11,"serialize","","",1,[[],["result",4]]],[11,"serialize","","",5,[[],["result",4]]],[11,"serialize","","",4,[[],["result",4]]],[11,"serialize","","",3,[[],["result",4]]],[11,"serialize","","",8,[[],["result",4]]],[11,"serialize","","",6,[[],["result",4]]],[11,"deserialize","","",1,[[],["result",4]]],[11,"deserialize","","",5,[[],["result",4]]],[11,"deserialize","","",4,[[],["result",4]]],[11,"deserialize","","",3,[[],["result",4]]],[11,"deserialize","","",8,[[],["result",4]]],[11,"deserialize","","",6,[[],["result",4]]],[11,"allow_all","","Returns a <code>Permisions</code> instance constructed with […",7,[[]]],[11,"allowed_to","","Evaluate whether the <code>action</code> is allowed to be taken upon …",7,[[["asref",8]],["bool",15]]],[11,"allow_all","","Returns a statement that allows [<code>ActionNameList::All</code>] …",5,[[]]],[11,"to_owned","","Convert this identifier to an un-borrowed identifier.",4,[[],["identifier",4]]],[11,"to_owned","","Convert a borrowed name to an un-borrwed name.",8,[[],["resourcename",3]]],[11,"any","","Creates a <code>ResourceName</code> that matches any identifier.",8,[[]]],[11,"named","","Creates a <code>ResourceName</code> with <code>name</code>.",8,[[["identifier",4],["into",8]]]],[11,"and","","Adds another name segment.",8,[[["identifier",4],["into",8]]]]],"p":[[8,"Action"],[3,"ActionName"],[8,"Dispatcher"],[4,"ActionNameList"],[4,"Identifier"],[3,"Statement"],[3,"PermissionDenied"],[3,"Permissions"],[3,"ResourceName"]]},\
"actionable_macros":{"doc":"Macros for the <code>actionable</code> API framework.","i":[[24,"Action","actionable_macros","Derives the <code>actionable::Action</code> trait.",null,null],[24,"Actionable","","Derives a set of traits that can be used to implement a …",null,null],[24,"Dispatcher","","Derives the <code>Dispatcher</code> trait.",null,null]],"p":[]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);