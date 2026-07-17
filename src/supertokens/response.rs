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

use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct SigninResponse {
    pub status: String,
    pub recipeUserId: Option<String>,
}

// #CreateSessionResponse
// {
//   "status": "OK",
//   "session": {
//     "handle": "68en6gd6-865b-4af6-ba00-96e5c153257d",
//     "userId": "fa7a0841-b533-4478-95533-0fde890c3483",
//     "userDataInJWT": {
//       "test": 123
//     },
//     "tenantId": "customer1",
//     "recipeUserId": "fa7a0841-b533-4478-95533-0fde890c3483"
//   },
//   "accessToken": {
//     "token": "ZTRiOTBjNz...jI5MTZlODkxw",
//     "expiry": 1637262633029,
//     "createdTime": 1637262633029
//   },
//   "refreshToken": {
//     "token": "ZTRiOTBjNz...jI5MTZlODkxw",
//     "expiry": 1637262633029,
//     "createdTime": 1637262633029
//   },
//   "antiCsrfToken": "ZTRiOTBjNz...jI5MTZlODkxw"
// }

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct CreateSessionResponse {
    pub status: String,
    pub accessToken: Option<AccessToken>,
    pub refreshToken: Option<AccessToken>,
}

#[derive(Deserialize, Serialize)]
pub struct AccessToken {
    token: String,
}
