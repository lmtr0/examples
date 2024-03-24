/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { Pet } from '../models/Pet';

import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';

export class DefaultService {

    /**
     * @param requestBody
     * @returns Pet OK
     * @throws ApiError
     */
    public static postPets(
        requestBody: Pet,
    ): CancelablePromise<Pet> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/pets',
            body: requestBody,
            mediaType: 'application/json',
        });
    }

}
