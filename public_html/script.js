/*
 * GLOBAL STUFF
 */

    //API KEY
    //Yes I know you shouldn't do this.
    let apiKey = "JJkp7tmEv369X4N5Ds3QsaUnWHCFn0cm4eLiDjby";



    //Make two global arrays to hold the mouse click vals.
    let x_vals = new Array();
    let y_vals = new Array();

    //Make a global dict of goals.
    let goalsMap = new Map();
    //make a str for global log.
    let statusLog = "";

/*
 * FUNCTIONS
 */

    function add_goal(event){
    var goalName = document.getElementById("goalName").value;
    var goalDescription = document.getElementById("goalDescription").value;
    goalsMap.set(goalName, goalDescription); 
    }

    function find_goal(event){
    console.log("Looking for goal...");
        var goalName = document.getElementById("goalSearch").value;
        //clear out the result box.
        document.getElementById("searchBox").value = "nothing";
        //now find the results.
        let searchResults = "";
        if(goalsMap.has(goalName)) {
            searchResults += goalName;
            searchResults += ": ";
            searchResults += goalsMap.get(goalName);       
        }
        else {
            searchResults += "0 results";    
        }
        document.getElementById("searchBox").value = searchResults;
    }

    function dark_mode(event) {
        var element = document.body;
        var box_one = document.getElementById("goals");
        var box_two = document.getElementById("settings");
        var box_three = document.getElementById("add");
        box_one.classList.toggle("dark-mode");
        box_two.classList.toggle("dark-mode");
        box_three.classList.toggle("dark-mode");
        element.classList.toggle("dark-mode"); 
    }

    // coordinate function that calculate the X
     // coordinate and Y coordinates
     function coordinate(event) {
        // clientX gives horizontal coordinate
        var x = event.clientX;
        // clientY gives vertical coordinates
        var y = event.clientY;
        document.getElementById("X").value = x;
        document.getElementById("Y").value = y;
        x_vals.push(x);
        y_vals.push(y);
    }

/*
   SECTION FOR POST REQUESTS
*/

    function send_comment(event) {
        //First check if it was the enter key that was pressed.
        if(event.keyCode != 13){
            return;
        }
        else{
            //get the value from the textbox.
            let text = document.getElementById("chatBoxInput").value;
            text += "\n\n"
            //construct the json
            json_data = '{ "text": ' + text + ', "webpage": "goaltracker"}';

            let xhr = new XMLHttpRequest();
            let url = "https://vl0n4nkzre.execute-api.us-east-1.amazonaws.com/dev/comments"; 
            
            //send that stuff.
            //Clear the chatbox input
            //open a connection
            xhr.open("POST", url, true);

            //set the request header
                xhr.setRequestHeader("Content-Type", "application/json");
                xhr.setRequestHeader("x-api-key", apiKey); 
            // Sending data with the request
            xhr.send(json_data);
                // Create a state change callback
                xhr.onreadystatechange = function () {
                    if (xhr.readyState === 4 && xhr.status === 200) {
      
                        // Print received data from server
                        console.log("posted comment.");
                    }
                };       
        }
        //Now we update the chatbox.
        get_comments();
    }


    function get_comments() {
            let xhr = new XMLHttpRequest();
            let url = "https://vl0n4nkzre.execute-api.us-east-1.amazonaws.com/dev/comments";  
            xhr.open("POST", url, true);

            //clear it out.
            document.getElementById("chatBoxOutput").value = "";

            //set the request header
                xhr.setRequestHeader("Content-Type", "application/json");
                xhr.setRequestHeader("x-api-key", apiKey); 
            // Sending data with the request
            xhr.send(json_data);
                // Create a state change callback
                xhr.onreadystatechange = function () {
                    if (xhr.readyState === 4 && xhr.status === 200) {
      
                        // Print received data from server
                        //body.innerHTML = this.responseText;
                        document.getElementById("chatBoxOutput").value = this.responseText;
                        console.log("response:", this.responseText); 
                    }
                };       
    }


    function test_comment(event) {
        if(event.keyCode != 13) {
            return;
        }
        let input = document.getElementById("chatBoxInput").value;
        input += "\n\n";
        document.getElementById("chatBoxOutput").value += input;
        document.getElementById("chatBoxInput").value = "";
    }

/*
 * SECTION FOR USING HEATMAP
 */

    function post_heatmap(event) {

        //Format stuff.
        var json_data = {
            "x_vals": x_vals,
            "y_vals": y_vals,
            "x_res": window.screen.availWidth,
            "y_res": window.screen.availHeight
            };

        json_data = JSON.stringify(json_data);       
        //Create a xhr object.
        let xhr = new XMLHttpRequest();
        let url = "/heatmap";

        //open a connection
        xhr.open("POST", url, true);

        //set the request header
            xhr.setRequestHeader("Content-Type", "application/json");
  
            // Create a state change callback
            xhr.onreadystatechange = function () {
                if (xhr.readyState === 4 && xhr.status === 200) {
  
                    // Print received data from server
                    //body.innerHTML = this.responseText;
                    document.getElementById("mainBody").innerHTML = this.responseText;
  
                }
            };
  
            // Converting JSON data to string
            //var data = JSON.stringify({ "name": name.value, "email": email.value });
  
            // Sending data with the request
            xhr.send(json_data);
        
        
    }
