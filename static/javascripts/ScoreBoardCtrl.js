angular.module('Connect4App').controller('ScoreBoardCtrl', ScoreBoardCtrl);


angular.module('Connect4App').factory('postService', function($resource){
    return $resource('/games');
});


function ScoreBoardCtrl(postService, $scope, $rootScope){

    $scope.games = postService.query();
    
};