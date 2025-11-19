import React from 'react';
import { Route, Routes } from 'react-router-dom';

import Home from './pages/Home';
import Login from './pages/Login';

const App = () => {
    return (
        <div id="AppRoot">

            <Routes>
                <Route path="/" element={<Home />}
            </Routes>
        </div>
    );
}

export default App;
