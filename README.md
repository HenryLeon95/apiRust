# API-RUST

Api programmed in the Rust language with connection to a MySql database hosted on GCP.<br>
To test each piece of code, you can comment and uncomment the blocks to interact with them and practice as you like.<br>
You can try the code located in the file example1.rs and example2.rs
<br><br>

## Part 1.
In this series of posts I will explore how to access MySQL from Rust. We will start simple and in later parts finish with developing a web service API.<br><br>
The MySQL client API is quite easy to use. I love the way it converts data to whatever type you want to work with. Here we have used i32, String, and NaieveDate. This should cover the most common data types used in database programming.
<br><br>

## Part 2.
In part 1 I discussed how to do basic query. In this article we will learn to do inserts and updates. We will continue to work with the PUBLICATION table.<br><br>
In this part we learned to do insert, update and delete. Go to Part III where we will build a proper data access layer.
<br><br>

## Part 3.
In Part II we explored how to insert, update and delete rows. In this part we will build a database access layer. It will be a set of functions that perform database access.<br>
We will continue to work with the PUBLICATION table we created earlier.<br><br>
Most of the code here is pretty simple. Biggest challenge was to return all possible outcomes from a database call. Rust does not have exception like Java or C++. We must reply on Result and Error to convey errors.<br>
Go to Part IV where we will expose the data access layer using a web service.