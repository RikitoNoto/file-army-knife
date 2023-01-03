import Head from 'next/head'
import { open } from "@tauri-apps/api/dialog";
import styles from '/styles/line_counter.module.scss'
import {IIconProps, IconButton, initializeIcons, Pivot, PivotItem, ProgressIndicator} from '@fluentui/react'
import 'bootstrap/dist/css/bootstrap.min.css';
import {Button, Card, Col, Container, Form, InputGroup, Row} from "react-bootstrap"
import Header from 'components/layouts/header'
import SearchCard from 'components/elements/line_counter/search_card'
import React, { ReactElement } from "react";
import { invoke } from '@tauri-apps/api/tauri';

initializeIcons();

type Props = {
}

interface State{
  search_cards: ReactElement[];
}

const PLAY_ICON: IIconProps = {iconName: "Play"};

class LineCounterPage  extends React.Component<Props, State> {

  private _search_cards: ReactElement[] = [];

  constructor(props: Props) {
    super(props);
    this._search_cards.push(<SearchCard gridSize={12}></SearchCard>)
    this.state = {
      search_cards: this._search_cards,
    };
    console.log(this);
  }

  componentDidMount() {
    console.log("call");

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
              {this.state.search_cards}
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

  onClickAddButton = () => {
    console.log(this._search_cards);
    console.log(this.state);
    this._search_cards.push(<SearchCard gridSize={12}></SearchCard>);
    // this.state.search_cards.push(<SearchCard gridSize={12}></SearchCard>);
    this.setState({
      search_cards: this._search_cards.concat(),
    });
  }

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
