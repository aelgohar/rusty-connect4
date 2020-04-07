angular.module('Connect4App').controller('tootHumanController', tootHumanController);

app.factory('postService', function($resource){
    return $resource('/games');
});

function tootHumanController(postService, $scope, $rootScope){
	$scope.games=[];
	$scope.newGame= {gameNumber:'100', gameType:'TOOT-OTTO', Player1Name: '', Player2Name: '', WinnerName: 'Unfinished Game', GameDate: Date.now(), Label:''};
	Array.prototype.clone = function () {
	    var arr = [], i;
	    for (i = 0; i < this.length; i++) {
	        arr[i] = this[i].slice();
	    }
	    return arr;
	};
	$scope.Game=function(){

	var target = document.getElementById('gameboard');
	var button = document.getElementById('startbutton');
	button.disabled = true;	

	var target1 = document.getElementById('textbox1');
	var target2 = document.getElementById('textbox2');

	//target1.readOnly = true;
	//target2.readOnly = true;

    var that = this;
    this.map = [];
    this.dummyMap = [];
    this.paused = false;
    this.won = false;
    this.rejectClick = false;
    this.move = 0;
    this.aiHistory = [];

    this.initOnceDone = false;
    /**
     * Only initalize once for these functions, can prevent race condition
     */
    this.initOnce = function () {
        if (this.initOnceDone) {
            return false;
        }

        this.canvas = document.getElementsByTagName("canvas")[0];
        this.canvas.addEventListener('click', function (e) {
           // alert($scope.newGame.Label);
           if($scope.newGame.Label === ''){
            return false;
           }
            that.onclick(that.canvas, e);
        });
        this.context = this.canvas.getContext('2d');
        this.context.font = "bold 25px serif";
        this.initOnceDone = true;
    };

    this.init = function () {
        this.map = [];
        this.dummyMap = [];
        this.paused = false;
        this.won = false;
        this.rejectClick = false;
        this.move = 0;
        this.aiHistory = [];
        this.initOnce();

        var i, j;
        for (i = 0; i <= 6; i++) {
            this.map[i] = [];
            this.dummyMap[i] = [];
            for (j = 0; j <= 7; j++) {
                this.map[i][j] = 0;
                this.dummyMap[i][j] = 0;
            }
        }
        this.clear();
        this.drawMask();
        this.print();
    };

    this.playerMove = function () {
        if (this.move % 2 === 0) {
            return 1;
        }
        return -1;
    };

    this.print = function () {
        var i, j, msg, dummymsg="";
        msg = "\n";
        msg += "Move: " + this.move;
        msg += "\n";
        for (i = 0; i < 6; i++) {
            for (j = 0; j < 7; j++) {
                msg += " " + this.map[i][j];
                dummymsg += " " + this.dummyMap[i][j];
            }
            msg += "\n";
            dummymsg += "\n";
        }
        console.log(msg);
        console.log(dummymsg);
    };

    this.printState = function (state) {
        var i, j, msg = "\n";
        for (i = 0; i < 6; i++) {
            for (j = 0; j < 7; j++) {
                msg += " " + state[i][j];
            }
            msg += "\n";
        }
        console.log(msg);
    };

    this.win = function (player) {
        this.paused = true;
        this.won = true;
        this.rejectClick = false;
        var msg = null;
        if (player > 0) {
            msg = $scope.newGame.Player1Name + " wins";
            $scope.newGame.WinnerName=$scope.newGame.Player1Name;
        } else if (player < 0) {
            msg = $scope.newGame.Player2Name + " wins";
            $scope.newGame.WinnerName=$scope.newGame.Player2Name;
        } else {
            msg = "It's a draw";
            $scope.newGame.WinnerName='Draw';
        }
        msg += " - Click on game board to reset";
        this.context.save();
        this.context.font = '14pt sans-serif';
        this.context.fillStyle = "#111";
        this.context.fillText(msg, 150, 20);
        //this.context.restore();

        postService.save($scope.newGame, function(){

            console.log("succesfully saved");
        });

        this.canvas = document.getElementsByTagName("canvas")[0];
        
        this.canvas.addEventListener('click', function (e) {
            location.reload();
            
        });
        //this.context.restore();
        button.disabled = false;

        console.info(msg);
    };
    this.fillMap = function (state, column, value) {
        var tempMap = state.clone();
        if (tempMap[0][column] !== 0 || column < 0 || column > 6) {
            return -1;
        }

        var done = false,
            row = 0,
            i;
        for (i = 0; i < 5; i++) {
            if (tempMap[i + 1][column] !== 0) {
                done = true;
                row = i;
                break;
            }
        }
        if (!done) {
            row = 5;
        }
        tempMap[row][column] = value;
        return tempMap;

    };

    this.action = function (column, callback) {
        if (this.paused || this.won) {
            return 0;
        }
        if (this.map[0][column] !== 0 || column < 0 || column > 6) {
            return -1;
        }

        var done = false;
        var row = 0, i;
        for (i = 0; i < 5; i++) {
            if (this.map[i + 1][column] !== 0) {
                done = true;
                row = i;
                break;
            }
        }
        if (!done) {
            row = 5;
        }
        this.animate(column, this.playerMove(this.move), row, 0, function () {
            that.map[row][column] = that.playerMove(that.move);
            that.dummyMap[row][column] = $scope.newGame.Label;
            that.move++;
            that.draw();
            that.check();
            that.print();
            //callback();
        });
        this.paused = true;
        return 1;
    };

    this.check = function () {
        var i, j, k;
        var temp_r = 0, temp_b = 0, temp_br = 0, temp_tr = 0;
        var temp_r1 = [], temp_b1 = [], temp_br1 = [], temp_br2 =[];
        for (i = 0; i < 6; i++) {
            for (j = 0; j < 7; j++) {
               /* temp_r = 0;
                temp_b = 0;
                temp_br = 0;
                temp_tr = 0; */

                temp_r1[0] = 0; temp_r1[1] = 0; temp_r1[2] = 0; temp_r1[3] = 0;
                temp_b1[0] = 0; temp_b1[1] = 0; temp_b1[2] = 0; temp_b1[3] = 0;
                temp_br1[0] = 0; temp_br1[1] = 0; temp_br1[2] = 0; temp_br1[3] = 0;
                temp_br2[0] = 0; temp_br2[1] = 0; temp_br2[2] = 0; temp_br2[3] = 0;
                for (k = 0; k <= 3; k++) {
                    //from (i,j) to right
                    if (j + k < 7) {
                      //  temp_r += this.map[i][j + k];
                      //temp_r1[k]=this.map[i][j+k];
                      temp_r1[k]=this.dummyMap[i][j+k];

                    }
                    //from (i,j) to bottom
                    if (i + k < 6) {
                       // temp_b += this.map[i + k][j];
                       //temp_b1[k] = this.map[i + k][j];
                       temp_b1[k] = this.dummyMap[i + k][j];
                    }

                    //from (i,j) to bottom-right
                    if (i + k < 6 && j + k < 7) {
                        //temp_br += this.map[i + k][j + k];
                        //temp_br1[k] = this.map[i + k][j + k];
                        temp_br1[k] = this.dummyMap[i + k][j + k];
                    }

                    //from (i,j) to top-right
                    if (i - k >= 0 && j + k < 7) {
                       // temp_tr += this.map[i - k][j + k];
                       //temp_br2[k] = this.map[i - k][j + k];
                       temp_br2[k] = this.dummyMap[i - k][j + k];
                    }
                }

                if(temp_r1[0] === 'T' && temp_r1[1] === 'O' && temp_r1[2] === 'O' && temp_r1[3] === 'T'){
                    this.win(1);
                }
                else if(temp_r1[0] === 'O' && temp_r1[1] === 'T' && temp_r1[2] === 'T' && temp_r1[3] === 'O'){
                    this.win(-1);
                }
                else if(temp_b1[0] === 'T' && temp_b1[1] ==='O' && temp_b1[2] === 'O' && temp_b1[3] === 'T'){
                    this.win(1);
                }
                else if(temp_b1[0] === 'O' && temp_b1[1] === 'T' && temp_b1[2] === 'T' && temp_b1[3] === 'O'){
                    this.win(-1);
                }
                else if(temp_br1[0] === 'T' && temp_br1[1] === 'O' && temp_br1[2] === 'O' && temp_br1[3] === 'T'){
                    this.win(1);
                }
                else if(temp_br1[0] ==='O' && temp_br1[1] === 'T' && temp_br1[2] === 'T' && temp_br1[3] === 'O'){
                    this.win(-1);
                }
                else if(temp_br2[0] === 'T' && temp_br2[1] ==='O' && temp_br2[2] === 'O' && temp_br2[3] === 'T'){
                    this.win(1);
                }
                else if(temp_br2[0] === 'O' && temp_br2[1] === 'T' && temp_br2[2] === 'T' && temp_br2[3] === 'O'){
                    this.win(-1);
                }


            }
        }
        // check if draw
        if ((this.move === 42) && (!this.won)) {
            this.win(0);
        }
    };

    this.drawCircle = function (x, y, r, fill, stroke, text) {
        this.context.save();
        this.context.fillStyle = fill;
        this.context.strokeStyle = stroke;
        this.context.beginPath();
        this.context.arc(x, y, r, 0, 2 * Math.PI, false);
        this.context.fill();
        this.context.font = "bold 25px serif";
        this.context.restore();
        this.context.fillText(text, x-8.5, y+8);
    };
    
    this.drawMask = function () {
        // draw the mask
        // http://stackoverflow.com/questions/6271419/how-to-fill-the-opposite-shape-on-canvas
        // -->  http://stackoverflow.com/a/11770000/917957

        this.context.save();
        this.context.fillStyle = "#00bfff";
        this.context.beginPath();
        var x, y;
        for (y = 0; y < 6; y++) {
            for (x = 0; x < 7; x++) {
                this.context.arc(75 * x + 100, 75 * y + 50, 25, 0, 2 * Math.PI);
                this.context.rect(75 * x + 150, 75 * y, -100, 100);
            }
        }
        this.context.fill();
        this.context.restore();
       // this.context.fillText("T", x, y);
    };

    this.draw = function () {
        var x, y;
        var fg_color;
        
        for (y = 0; y < 6; y++) {

            for (x = 0; x < 7; x++) {
                var text="";
                fg_color = "transparent";
                if (this.map[y][x] >= 1 && this.dummyMap[y][x] === 'T') {
                    fg_color = "#99ffcc";
                    text="T";
                    //text=$scope.newGame.Label;
                }else if (this.map[y][x] >= 1 && this.dummyMap[y][x] === 'O') {
                    fg_color = "#99ffcc";
                    text="O";
                   // text=$scope.newGame.Label;
                }else if (this.map[y][x] <= -1 && this.dummyMap[y][x] === 'T') {
                    fg_color = "#ffff99";
                    text="T";
                   // text=$scope.newGame.Label;
                }else if (this.map[y][x] <= -1 && this.dummyMap[y][x] === 'O') {
                    fg_color = "#ffff99";
                    text="O";
                   // text=$scope.newGame.Label;
                }

                this.drawCircle(75 * x + 100, 75 * y + 50, 25, fg_color, "black", text);
            }
        }
    };
    this.clear = function () {
        this.context.clearRect(0, 0, this.canvas.width, this.canvas.height);
    };
    this.animate = function (column, move, to_row, cur_pos, callback) {
        var text = "";
        var fg_color = "transparent";
        if (move >= 1) {
            fg_color = "#99ffcc";
            //text="T";
            text = $scope.newGame.Label;
        } else if (move <= -1) {
            fg_color = "#ffff99";
            //text="O";
            text = $scope.newGame.Label;
        }
        if (to_row * 75 >= cur_pos) {
            this.clear();
            this.draw();
            this.drawCircle(75 * column + 100, cur_pos + 50, 25, fg_color, "black", text);
            this.drawMask();
            window.requestAnimationFrame(function () {
                that.animate(column, move, to_row, cur_pos + 25, callback);
            });
        } else {
            callback();
        }
    };

    this.onregion = function (coord, x, radius) {
        if ((coord[0] - x)*(coord[0] - x) <=  radius * radius) {
            return true;
        }
        return false;
    };
    this.oncircle = function (coord, centerCoord, radius) {
        if ((coord[0] - centerCoord[0]) * (coord[0] - centerCoord[0]) <=  radius * radius
                && (coord[1] - centerCoord[1]) * (coord[1] - centerCoord[1]) <=  radius * radius) {
            return true;
        }
        return false;
    };

    this.onclick = function (canvas, e) {
        if (this.rejectClick) {
            return false;
        }
        if (this.won) {
            this.init();
            return false;
        }
        var rect = canvas.getBoundingClientRect(),
            x = e.clientX - rect.left,// - e.target.scrollTop,
            y = e.clientY - rect.top;// - e.target.scrollLeft;

        //console.log("(" + x + ", " + y + ")");
        var j, valid;
        for (j = 0; j < 7; j++) {
            if (this.onregion([x, y], 75 * j + 100, 25)) {
                // console.log("clicked region " + j);
                this.paused = false;
                this.action(j);

                
                if (valid === 1) { // give user retry if action is invalid
                    this.rejectClick = true;
                }
                break; //because there will be no 2 points that are clicked at a time
            }
        }
    };

    this.init();

		$scope.games.push($scope.newGame);

		//target1.disabled = true;
	  	//target2.disabled = true;	
	};
}