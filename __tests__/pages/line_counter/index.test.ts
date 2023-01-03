import { cleanup, fireEvent, render, screen } from '@testing-library/react';
import {describe, expect, test} from '@jest/globals';
import LineCounterPage from 'pages/line_counter/index';
import React, {ReactElement} from "react";

// describe('sum module', () => {
//     test('adds 1 + 2 to equal 3', () => {
//         expect(1+2).toBe(3);
//     });
// });

describe('add button', () => {
    test('should be add search card on click', async () => {
        const page = new LineCounterPage({});
        await render(page.render() as ReactElement);
        expect(screen.getAllByText('Start Text').length).toBe(1);

        await fireEvent.click(screen.getByText('+'));
        await cleanup();
        await render(page.render() as ReactElement);

        expect(screen.getAllByText('Start Text').length).toBe(2);
    });
});
