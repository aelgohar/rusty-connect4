angular.module('Connect4App').controller('ScoresCtrl', ScoresCtrl);


angular.module('Connect4App').factory('postService', function($resource){
    return $resource('/games');
});

function ScoresCtrl(postService, $scope, $http, $rootScope){

    $scope.games = postService.query();

};