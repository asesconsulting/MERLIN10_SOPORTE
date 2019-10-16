<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Enrichment_Contactabilidad</name>
   <tag></tag>
   <elementGuidId>f7ecb874-8842-44ff-8597-2a2c40183ef4</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:soap=&quot;http://soap2.enrichment.ases.com/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;soap:enrichment>
         &lt;!--Optional:-->
         &lt;enrichment>
            &lt;!--Optional:-->
            &lt;clientAccessCode>aea243aba41084aa098b3a70eeb63ddf&lt;/clientAccessCode>
            &lt;!--Optional:-->
            &lt;customAdaptersKeys>
               &lt;!--Zero or more repetitions:-->
               &lt;customAdapters> &lt;/customAdapters>
            &lt;/customAdaptersKeys>
            &lt;!--Optional:-->
            &lt;xPersonEnrichment>
               &lt;!--Optional:-->
               &lt;level1>AR&lt;/level1>
               &lt;!--Optional:-->
               &lt;documentType> &lt;/documentType>
               &lt;!--Optional:-->
               &lt;documentNumber>&lt;/documentNumber>
               &lt;!--Optional:-->
               &lt;tributaryType> &lt;/tributaryType>
               &lt;!--Optional:-->
               &lt;tributaryNumber>20217886660&lt;/tributaryNumber>
               &lt;!--Optional:-->
               &lt;lastName> &lt;/lastName>
               &lt;!--Optional:-->
               &lt;name> &lt;/name>
               &lt;!--Optional:-->
               &lt;gender> &lt;/gender>
               &lt;!--Optional:-->
               &lt;birthDate> &lt;/birthDate>
               &lt;!--Optional:-->
               &lt;queryId>32&lt;/queryId>
            &lt;/xPersonEnrichment>
         &lt;/enrichment>
      &lt;/soap:enrichment>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>enrichment</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.Enrichment_ARG2</defaultValue>
      <description></description>
      <id>c922b64d-897a-482b-a089-fc0c1cdfbd49</id>
      <masked>false</masked>
      <name>Enrichment_ARG2</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



WS.verifyElementText(response, 'enrichmentResponse.return.status', 'EN')
WS.verifyElementText(response, 'enrichmentResponse.return.statusReason', 'SM')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Tipo&lt;/name>&lt;label>&lt;/label>&lt;value>FISICA&lt;/value>&lt;index>0&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>NumeroTributario&lt;/name>&lt;label>&lt;/label>&lt;value>20217886660&lt;/value>&lt;index>1&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>TipoTributario&lt;/name>&lt;label>&lt;/label>&lt;value>CUIT PERSONA FISICA&lt;/value>&lt;index>2&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>TipoTributarioCodificado&lt;/name>&lt;label>&lt;/label>&lt;value>80&lt;/value>&lt;index>3&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>TipoTributarioAfip&lt;/name>&lt;label>&lt;/label>&lt;value>F&lt;/value>&lt;index>4&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>NumeroDocumento&lt;/name>&lt;label>&lt;/label>&lt;value>21788666&lt;/value>&lt;index>5&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>TipoDocumento&lt;/name>&lt;label>&lt;/label>&lt;value>DNI&lt;/value>&lt;index>6&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>TipoDocumentoCodificado&lt;/name>&lt;label>&lt;/label>&lt;value>96&lt;/value>&lt;index>7&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>NombreCompleto&lt;/name>&lt;label>&lt;/label>&lt;value>DENIARD FERNANDO LUIS&lt;/value>&lt;index>8&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Apellidos&lt;/name>&lt;label>&lt;/label>&lt;value>DENIARD&lt;/value>&lt;index>9&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Nombres&lt;/name>&lt;label>&lt;/label>&lt;value>FERNANDO LUIS&lt;/value>&lt;index>10&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Genero&lt;/name>&lt;label>&lt;/label>&lt;value>M&lt;/value>&lt;index>11&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>FechaNacimiento&lt;/name>&lt;label>&lt;/label>&lt;value>11/08/1970&lt;/value>&lt;index>12&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Fallecido&lt;/name>&lt;label>&lt;/label>&lt;value>NO&lt;/value>&lt;index>13&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>FechaFallecimiento&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>14&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>EstadoCivil&lt;/name>&lt;label>&lt;/label>&lt;value>C&lt;/value>&lt;index>15&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Nacionalidad&lt;/name>&lt;label>&lt;/label>&lt;value>ARGENTINA&lt;/value>&lt;index>16&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Residencia&lt;/name>&lt;label>&lt;/label>&lt;value>ARGENTINA &lt;/value>&lt;index>17&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Votante&lt;/name>&lt;label>&lt;/label>&lt;value>SI&lt;/value>&lt;index>18&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>DocumentoReemplazo&lt;/name>&lt;label>&lt;/label>&lt;value>&lt;/value>&lt;index>19&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Estado&lt;/name>&lt;label>&lt;/label>&lt;value>CO&lt;/value>&lt;index>20&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Motivo&lt;/name>&lt;label>&lt;/label>&lt;value>SM&lt;/value>&lt;index>21&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>TipoGeo&lt;/name>&lt;label>&lt;/label>&lt;value>1&lt;/value>&lt;index>22&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Pais&lt;/name>&lt;label>&lt;/label>&lt;value>ARGENTINA&lt;/value>&lt;index>23&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Provincia&lt;/name>&lt;label>&lt;/label>&lt;value>CAPITAL FEDERAL&lt;/value>&lt;index>24&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Partido&lt;/name>&lt;label>&lt;/label>&lt;value>CAPITAL FEDERAL&lt;/value>&lt;index>25&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Localidad&lt;/name>&lt;label>&lt;/label>&lt;value>CIUDAD AUTONOMA BUENOS AIRES&lt;/value>&lt;index>26&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Barrio&lt;/name>&lt;label>&lt;/label>&lt;value>NUÑEZ&lt;/value>&lt;index>27&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Calle&lt;/name>&lt;label>&lt;/label>&lt;value>AVDA DEL LIBERTADOR&lt;/value>&lt;index>28&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Numero&lt;/name>&lt;label>&lt;/label>&lt;value>7270&lt;/value>&lt;index>29&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Piso&lt;/name>&lt;label>&lt;/label>&lt;value>2&lt;/value>&lt;index>30&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Depto&lt;/name>&lt;label>&lt;/label>&lt;value>A&lt;/value>&lt;index>31&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>CodigoPostal&lt;/name>&lt;label>&lt;/label>&lt;value>1429&lt;/value>&lt;index>32&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>CPA&lt;/name>&lt;label>&lt;/label>&lt;value>C1429BMS&lt;/value>&lt;index>33&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Longitud&lt;/name>&lt;label>&lt;/label>&lt;value>-58.457995&lt;/value>&lt;index>34&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Latitud&lt;/name>&lt;label>&lt;/label>&lt;value>-34.546420&lt;/value>&lt;index>35&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>EntreCalle1&lt;/name>&lt;label>&lt;/label>&lt;value>JUANA AZURDUY DE PADILLA&lt;/value>&lt;index>36&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>EntreCalle2&lt;/name>&lt;label>&lt;/label>&lt;value>MANUELA PEDRAZA&lt;/value>&lt;index>37&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Tipo&lt;/name>&lt;label>&lt;/label>&lt;value>F&lt;/value>&lt;index>38&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Observaciones&lt;/name>&lt;label>&lt;/label>&lt;value> &lt;/value>&lt;index>39&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Edificio&lt;/name>&lt;label>&lt;/label>&lt;value>SI&lt;/value>&lt;index>40&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Casa&lt;/name>&lt;label>&lt;/label>&lt;value>NO&lt;/value>&lt;index>41&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Comercio&lt;/name>&lt;label>&lt;/label>&lt;value>NO&lt;/value>&lt;index>42&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Puerta&lt;/name>&lt;label>&lt;/label>&lt;value>SI&lt;/value>&lt;index>43&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Estado&lt;/name>&lt;label>&lt;/label>&lt;value>CO&lt;/value>&lt;index>44&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Motivo&lt;/name>&lt;label>&lt;/label>&lt;value>SM&lt;/value>&lt;index>45&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>TipoGeo&lt;/name>&lt;label>&lt;/label>&lt;value>1&lt;/value>&lt;index>46&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Pais&lt;/name>&lt;label>&lt;/label>&lt;value>ARGENTINA&lt;/value>&lt;index>47&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Provincia&lt;/name>&lt;label>&lt;/label>&lt;value>CAPITAL FEDERAL&lt;/value>&lt;index>48&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Partido&lt;/name>&lt;label>&lt;/label>&lt;value>CAPITAL FEDERAL&lt;/value>&lt;index>49&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Localidad&lt;/name>&lt;label>&lt;/label>&lt;value>CIUDAD AUTONOMA BUENOS AIRES&lt;/value>&lt;index>50&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Calle&lt;/name>&lt;label>&lt;/label>&lt;value>NUÑEZ&lt;/value>&lt;index>51&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Numero&lt;/name>&lt;label>&lt;/label>&lt;value>4426&lt;/value>&lt;index>52&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Piso&lt;/name>&lt;label>&lt;/label>&lt;value>5&lt;/value>&lt;index>53&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Depto&lt;/name>&lt;label>&lt;/label>&lt;value>A&lt;/value>&lt;index>54&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>CodigoPostal&lt;/name>&lt;label>&lt;/label>&lt;value>1429&lt;/value>&lt;index>55&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>CPA&lt;/name>&lt;label>&lt;/label>&lt;value>C1429AWH&lt;/value>&lt;index>56&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Longitud&lt;/name>&lt;label>&lt;/label>&lt;value>-58.471696&lt;/value>&lt;index>57&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Latitud&lt;/name>&lt;label>&lt;/label>&lt;value>-34.542244&lt;/value>&lt;index>58&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>EntreCalle1&lt;/name>&lt;label>&lt;/label>&lt;value>CORONEL MANUEL EDUARDO ARIAS&lt;/value>&lt;index>59&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>EntreCalle2&lt;/name>&lt;label>&lt;/label>&lt;value>RAMALLO&lt;/value>&lt;index>60&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Tipo&lt;/name>&lt;label>&lt;/label>&lt;value> &lt;/value>&lt;index>61&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Observaciones&lt;/name>&lt;label>&lt;/label>&lt;value> &lt;/value>&lt;index>62&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Edificio&lt;/name>&lt;label>&lt;/label>&lt;value>SI&lt;/value>&lt;index>63&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Casa&lt;/name>&lt;label>&lt;/label>&lt;value>NO&lt;/value>&lt;index>64&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Comercio&lt;/name>&lt;label>&lt;/label>&lt;value>NO&lt;/value>&lt;index>65&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Puerta&lt;/name>&lt;label>&lt;/label>&lt;value>SI&lt;/value>&lt;index>66&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Prefijo&lt;/name>&lt;label>&lt;/label>&lt;value>011&lt;/value>&lt;index>67&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Numero&lt;/name>&lt;label>&lt;/label>&lt;value>47035095&lt;/value>&lt;index>68&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Tipo&lt;/name>&lt;label>&lt;/label>&lt;value>FIJO&lt;/value>&lt;index>69&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>NoLlame&lt;/name>&lt;label>&lt;/label>&lt;value>SI&lt;/value>&lt;index>70&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>TipoTelefono&lt;/name>&lt;label>&lt;/label>&lt;value>PERSONAL&lt;/value>&lt;index>71&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Operador&lt;/name>&lt;label>&lt;/label>&lt;value>TELECOM ARGENTINA STET FRANCE TELECOM S.A.&lt;/value>&lt;index>72&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Prefijo&lt;/name>&lt;label>&lt;/label>&lt;value>011&lt;/value>&lt;index>73&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Numero&lt;/name>&lt;label>&lt;/label>&lt;value>51796055&lt;/value>&lt;index>74&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Tipo&lt;/name>&lt;label>&lt;/label>&lt;value>MOVIL&lt;/value>&lt;index>75&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>NoLlame&lt;/name>&lt;label>&lt;/label>&lt;value>NO&lt;/value>&lt;index>76&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>TipoTelefono&lt;/name>&lt;label>&lt;/label>&lt;value> &lt;/value>&lt;index>77&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Operador&lt;/name>&lt;label>&lt;/label>&lt;value> &lt;/value>&lt;index>78&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Prefijo&lt;/name>&lt;label>&lt;/label>&lt;value>02320&lt;/value>&lt;index>79&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Numero&lt;/name>&lt;label>&lt;/label>&lt;value>300223&lt;/value>&lt;index>80&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Tipo&lt;/name>&lt;label>&lt;/label>&lt;value> &lt;/value>&lt;index>81&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>NoLlame&lt;/name>&lt;label>&lt;/label>&lt;value>NO&lt;/value>&lt;index>82&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>TipoTelefono&lt;/name>&lt;label>&lt;/label>&lt;value> &lt;/value>&lt;index>83&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Operador&lt;/name>&lt;label>&lt;/label>&lt;value> &lt;/value>&lt;index>84&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Prefijo&lt;/name>&lt;label>&lt;/label>&lt;value>011&lt;/value>&lt;index>85&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Numero&lt;/name>&lt;label>&lt;/label>&lt;value>51796064&lt;/value>&lt;index>86&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Tipo&lt;/name>&lt;label>&lt;/label>&lt;value>MOVIL&lt;/value>&lt;index>87&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>NoLlame&lt;/name>&lt;label>&lt;/label>&lt;value>NO&lt;/value>&lt;index>88&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>TipoTelefono&lt;/name>&lt;label>&lt;/label>&lt;value> &lt;/value>&lt;index>89&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Operador&lt;/name>&lt;label>&lt;/label>&lt;value> &lt;/value>&lt;index>90&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Prefijo&lt;/name>&lt;label>&lt;/label>&lt;value>011&lt;/value>&lt;index>91&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Numero&lt;/name>&lt;label>&lt;/label>&lt;value>40863915&lt;/value>&lt;index>92&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Tipo&lt;/name>&lt;label>&lt;/label>&lt;value>MOVIL&lt;/value>&lt;index>93&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>NoLlame&lt;/name>&lt;label>&lt;/label>&lt;value>NO&lt;/value>&lt;index>94&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>TipoTelefono&lt;/name>&lt;label>&lt;/label>&lt;value> &lt;/value>&lt;index>95&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Operador&lt;/name>&lt;label>&lt;/label>&lt;value> &lt;/value>&lt;index>96&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Prefijo&lt;/name>&lt;label>&lt;/label>&lt;value>011&lt;/value>&lt;index>97&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Numero&lt;/name>&lt;label>&lt;/label>&lt;value>48314563&lt;/value>&lt;index>98&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Tipo&lt;/name>&lt;label>&lt;/label>&lt;value>FIJO&lt;/value>&lt;index>99&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>NoLlame&lt;/name>&lt;label>&lt;/label>&lt;value>NO&lt;/value>&lt;index>100&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Operador&lt;/name>&lt;label>&lt;/label>&lt;value>PERSONAL&lt;/value>&lt;index>101&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Email&lt;/name>&lt;label>&lt;/label>&lt;value>fdeniard@gmail.com&lt;/value>&lt;index>102&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Tipo&lt;/name>&lt;label>&lt;/label>&lt;value>PERSONAL&lt;/value>&lt;index>103&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Email&lt;/name>&lt;label>&lt;/label>&lt;value>jdangiolo@gmail.com&lt;/value>&lt;index>104&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')
assertThat(response.getResponseText()).contains('&lt;element>&lt;name>Tipo&lt;/name>&lt;label>&lt;/label>&lt;value>PERSONAL&lt;/value>&lt;index>105&lt;/index>&lt;dataType>&lt;/dataType>&lt;/element>')</verificationScript>
   <wsdlAddress>${Enrichment_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
