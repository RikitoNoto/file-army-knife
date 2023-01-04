import Head from 'next/head'
import { open } from "@tauri-apps/api/dialog";
import styles from '/styles/line_counter.module.scss'
import {IIconProps, IconButton, initializeIcons, Pivot, PivotItem, ProgressIndicator} from '@fluentui/react'
import 'bootstrap/dist/css/bootstrap.min.css';
import {Button, Card, Col, Container, Form, InputGroup, Row} from "react-bootstrap"
import Header from 'components/layouts/header'
import SearchCard from 'components/elements/line_counter/search_card'
import React, { ReactElement, Key } from "react";
import { invoke } from '@tauri-apps/api/tauri';

initializeIcons();

type Props = {
}

interface State{
  search_cards: {[key: Key]: ReactElement};
}

const PLAY_ICON: IIconProps = {iconName: "Play"};

class LineCounterPage  extends React.Component<Props, State> {

  private _search_cards: {[key: Key]: ReactElement} = {};
  // private _search_cards: ReactElement[] = [];

  constructor(props: Props) {
    super(props);
    this.addCard();

    this.state = {
      search_cards: Object.assign({}, this._search_cards),
    };
  }

  componentDidMount() {
  }

  componentWillUnmount() {
  }


	render(): React.ReactNode {
    return (
      <div className={styles.container}>
        <Head>
          <title>Line counter</title>
          <link rel="icon" href="/favicon.ico" />
        </Head>

        <main className={styles.main}>
          <div className={`${styles.no_scroll_header}`}>
            <Header></Header>

            <div className={`${styles.top_bar}`}>
              <Container className={`${styles.path_container} col-11`}>
                <Row>
                  <Col xs={8}>
                    <InputGroup>
                      <Form.Control placeholder="input the path" className={`${styles.input_field__input_area}`}/>
                      <Button variant="outline-secondary" type='button' className={`${styles.input_field__dialog_button}`} onClick={this.onClickGetPathButton}>
                        ...
                      </Button>
                    </InputGroup>
                  </Col>

                  <Col xs={2} >
                    <IconButton iconProps={PLAY_ICON} onClick={this.onClickStartButton}></IconButton>
                  </Col>
                </Row>
              </Container>
            </div>

            <div className={`${styles.pane_separator}`}>
              <ProgressIndicator percentComplete={undefined} className={`${styles.progress}`}/>
            </div>
          </div>

          <Pivot className={`${styles.operation_pane}`}>
            <PivotItem headerText='conditions'>
              {
                Object.keys(this.state.search_cards).map((key)=>
                  this.state.search_cards[key]
                )
              }
              <Button variant="primary" type='button' className={`${styles.add_button} rounded-circle`} onClick={this.onClickAddButton}>+</Button>
            </PivotItem>
            <PivotItem headerText='result'>

            <Container>
              <Row className="justify-content-md-center">
                <Col xs={10}>
                <Card body className={`${styles.result_card}`}>
                  <p>Line count: 1000</p>
                </Card>
                </Col>
              </Row>
            </Container>
            </PivotItem>

          </Pivot>

        </main>

        <footer className={styles.footer}>
        </footer>
      </div>
    );
  }

  private addCard(){
    let key = Object.keys(this._search_cards).length + 1;
    let element = <SearchCard key={key} dictKey={key} gridSize={12} onDelete={this.onClickDeleteButton}></SearchCard>;
    this._search_cards[key] = element;
  }

  private onClickAddButton = () => {
    this.addCard();
    this.setState({
      search_cards: Object.assign({}, this._search_cards),
    })
  };

  private onClickDeleteButton = (card: SearchCard) =>{
    if(card.key){
      delete this._search_cards[card.key];
    }

    this.setState({
      search_cards: Object.assign({}, this._search_cards),
    })
  };

  onClickStartButton(): void{

  }

  onClickGetPathButton(): void{

    let properties = {
      defaultPath: '',
      directory: true,
    };

    open(properties).then((pathStr) => {
      console.log(invoke('get_enumrate_files', {path: pathStr, pattern: ""}).then(console.log).catch(console.error));
    });

  }

}

export default LineCounterPage;
