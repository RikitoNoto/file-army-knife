import { cleanup, fireEvent, render, screen } from '@testing-library/react';
import {describe, expect, test} from '@jest/globals';
import LineCounterPage from 'pages/line_counter/index';
import React, {ReactElement} from "react";

async function checkClick(page: React.Component, click_text: string){
    await fireEvent.click(screen.getByText(click_text));
    await cleanup();
    await render(page.render() as ReactElement);
}

describe('add button', () => {

    test('should be add search card on click', async () => {
        await cleanup();
        const page = new LineCounterPage({});
        await render(page.render() as ReactElement);
        expect(screen.getAllByText('Start Text').length).toBe(1);

        await checkClick(page, '+');

        expect(screen.getAllByText('Start Text').length).toBe(2);
    });
});

describe('delete button', () => {

    test('should be delete search card on click', async () => {
        await cleanup();
        const page = new LineCounterPage({});
        await render(page.render() as ReactElement);
        expect(screen.getAllByText('Start Text').length).toBe(1);

        await checkClick(page, 'Delete');

        expect(screen.getAllByText('Start Text').length).toBe(0);
    });
});
