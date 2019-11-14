# Text String 
$string = "Welcome to GeeksForGeeks"; 
  
# Let us use m operator to search  
# "to Ge" 
$string =~ m/to Ge/; 
  
# Printing the String 
print "blbl $1";
print "Before: $`\n"; 
print "Matched: $&\n"; 
print "After: $'\n"; 
