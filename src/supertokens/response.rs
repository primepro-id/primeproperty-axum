// #SigninResponse
// {
//   "status": "OK",
//   "user": {
//     "id": "fa7a0841-b533-4478-95533-0fde890c3483",
//     "isPrimaryUser": true,
//     "tenantIds": [
//       "customer1"
//     ],
//     "timeJoined": 1623918032231,
//     "emails": [
//       "johndoe@gmail.com"
//     ],
//     "phoneNumbers": [
//       "+14155552671"
//     ],
//     "thirdParty": [
//       {
//         "id": "google",
//         "userId": "106347997792363870000"
//       }
//     ],
//     "loginMethods": [
//       {
//         "tenantIds": [
//           "customer1"
//         ],
//         "recipeUserId": "fa7a0841-b533-4478-95533-0fde890c3483",
//         "verified": true,
//         "timeJoined": 1623918032231,
//         "recipeId": "emailpassword",
//         "email": "johndoe@gmail.com",
//         "phoneNumber": "+14155552671",
//         "thirdParty": {
//           "id": "google",
//           "userId": "106347997792363870000"
//         }
//       }
//     ]
//   },
//   "recipeUserId": "fa7a0841-b533-4478-95533-0fde890c3483"
// }
//

use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub(super) struct SigninResponse {
    status: String,
    loginMethods: Vec<LoginMethod>,
    recipeUserId: String,
}

#[derive(Deserialize)]
pub(super) struct LoginMethod {
    verified: bool,
}
