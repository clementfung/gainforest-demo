/*
 * Copyright 2017 The boardgame.io Authors.
 *
 * Use of this source code is governed by a MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
 */

import React from 'react';
import PropTypes from 'prop-types';
import './board.css';

class Board extends React.Component {
  static propTypes = {
    G: PropTypes.any.isRequired,
    ctx: PropTypes.any.isRequired,
    moves: PropTypes.any.isRequired,
    playerID: PropTypes.number,
    isSpectating: PropTypes.bool,
    isActive: PropTypes.bool,
    isMultiplayer: PropTypes.bool,
  };

  onClick = id => {
    this.props.moves.click_cell(id)
  };

  format (cellValue) {
    if (cellValue === -1) return '';
    return cellValue;
  }

  getCellClass (id) {
    switch (this.props.G.forest[id]) {
      case 1:
        return 'forested';
      case -1: 
        return 'deforested';
      default: 
        return 'active';  
    }
  }

  render() {
    let tbody = [];
    for (let i = 0; i < 4; i++) {
      let cells = [];

      for (let j = 0; j < 8; j++) {
        const id = 8 * i + j;

        let cellValue = '';
        if (this.props.G.cells[id] != 0) {
          cellValue = this.props.G.cells[id];
          cellValue = cellValue + '💲';
        }

        cells.push(
          <td
            key={id}
            className={this.getCellClass(id)}
            onClick={() => this.onClick(id)}
          >
            <img 
              src={"https://tiles.planet.com/basemaps/v1/planet-tiles/global_quarterly_" 
                + (2016 + Math.floor(this.props.G.time / 4)) 
                + "q" 
                + ((this.props.G.time % 4) + 1) 
                + "_mosaic/gmap/11/72" 
                + j
                + "/106" 
                + i 
                + ".png?api_key=d4d25171b85b4f7f8fde459575cba233"}
            >
            </img>
          </td>
        );
      }
      tbody.push(<tr key={i}>{cells}</tr>);
    }

    let rendered = (
      <div className="flex flex-column justify-center items-center">
        <table id="board">
          <tbody>{tbody}</tbody>
        </table>
        <td
            key={99}
            onClick={() => this.onClick(99)}
          >
          🛰    
        </td>
        <p> Time: {(2016 + Math.floor(this.props.G.time / 4)) + "q" + ((this.props.G.time % 4) + 1)} </p>
        <p> Balance: { this.props.G.stake } </p>
      </div>
    );
    return rendered;
  }
}

export default Board;
